use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Host, Request, State},
    http::Response,
    routing::{get, post},
    Json, RequestExt, Router,
};
use cloudstate_runtime::{
    blob_storage::CloudstateBlobStorage,
    cloudstate_extensions::cloudstate_extensions,
    extensions::cloudstate::{JavaScriptSpans, TransactionContext},
    gc::mark_and_sweep,
    permissions::CloudstatePermissions,
};
use deno_runtime::deno_permissions::PermissionCheckError;

use cloudstate_runtime::{extensions::cloudstate::ReDBCloudstate, v8_string_key};
use deno_core::JsRuntime;
use deno_core::*;
use deno_core::{
    futures::future::poll_fn, resolve_import, ModuleLoadResponse, ModuleLoader, ModuleSource,
    ModuleSourceCode, ModuleSpecifier, ModuleType, ResolutionKind,
};
use deno_fetch::FetchPermissions;
use deno_net::NetPermissions;
use deno_web::TimersPermission;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::rc::Rc;
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
};
use tracing::{debug, event, instrument};

#[cfg(test)]
mod tests;

pub struct CloudstateServer {
    pub cloudstate: ReDBCloudstate,
    pub blob_storage: CloudstateBlobStorage,
    pub router: Router,
}

struct CloudstateModuleLoader {
    lib: String,
}

impl ModuleLoader for CloudstateModuleLoader {
    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        ModuleLoadResponse::Sync(Ok(ModuleSource::new(
            ModuleType::JavaScript,
            ModuleSourceCode::String(self.lib.clone().into()),
            module_specifier,
            None,
        )))
    }
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, anyhow::Error> {
        Ok(resolve_import(specifier, referrer)?)
    }
}

impl CloudstateServer {
    pub async fn new(
        cloudstate: ReDBCloudstate,
        blob_storage: CloudstateBlobStorage,
        classes: &str,
        env: HashMap<String, String>,
        invalidate_endpoint: String,
    ) -> Self {
        let env_string = serde_json::to_string(&env).unwrap();

        execute_script(
            &include_str!("./initialize.js").replace("env_string", &env_string),
            classes,
            cloudstate.clone(),
            blob_storage.clone(),
        )
        .await;

        execute_script(
            "
            import { CloudstateInspectionCS } from './lib.js';
            registerCustomClass(CloudstateInspectionCS);
            if (getRoot('inspection') === undefined) {{
                setRoot('inspection', new CloudstateInspectionCS());
            }}
            ",
            include_str!("./inspection.js"),
            cloudstate.clone(),
            blob_storage.clone(),
        )
        .await;

        let app = Router::new()
            .route(
                "/cloudstate/instances/:id",
                get(fetch_request)
                    .post(fetch_request)
                    .patch(fetch_request)
                    .put(fetch_request)
                    .delete(fetch_request)
                    .head(fetch_request),
            )
            .route("/cloudstate/instances/:id/:method", post(method_request))
            .with_state(AppState {
                cloudstate: cloudstate.clone(),
                classes: classes.to_string(),
                env,
                invalidate_endpoint,
                blob_storage: blob_storage.clone(),
            });

        CloudstateServer {
            router: app,
            blob_storage: blob_storage,
            cloudstate,
        }
    }

    pub async fn gc(&self) -> anyhow::Result<()> {
        let db = self.cloudstate.get_database_mut();
        match mark_and_sweep(&db) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(anyhow!("Error running garbage collection: {:?}", e));
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ScriptResponseResult {
    Response { response: ResponseData },
    Error { error: ErrorData },
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    pub bytes: Vec<u8>,
    pub headers: Vec<(String, String)>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ErrorData {
    pub message: String,
    pub stack: String,
}

#[instrument(skip(id, state, host, request))]
async fn fetch_request(
    axum::extract::Path(id): axum::extract::Path<String>,
    State(state): State<AppState>,
    Host(host): Host,
    request: Request,
) -> axum::response::Response {
    let id = serde_json::to_string(&id).unwrap();
    let (parts, body) = request.into_parts();
    let http_method = parts.method.to_string();

    let headers = parts.headers;
    // TODO: find a way to not need the http:// prefix
    let uri = format!("http://{}{}", host, parts.uri.path());
    let uri = serde_json::to_string(&uri).unwrap();

    let headers = headers
        .iter()
        .map(|(key, value)| {
            format!(
                "{}: {}",
                serde_json::to_string(&key.to_string()).unwrap(),
                serde_json::to_string(&value.to_str().unwrap_or_default().to_string()).unwrap()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    let headers = format!("{{{}}}", headers);
    // let headers = serde_json::to_string(&headers).unwrap();

    let mut bytes = Vec::new();
    let mut stream = body.into_data_stream();
    while let Ok(Some(chunk)) = stream.try_next().await {
        bytes.extend_from_slice(&chunk);
    }

    let _bytes = format!(
        "[{}]",
        bytes
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let env_string = serde_json::to_string(&state.env).unwrap();
    let invalidate_endpoint = state.invalidate_endpoint.clone();

    let script = include_str!("./fetch_request.js")
        .replace("$ENV_STRING", &env_string)
        .replace("$INVALIDATE_ENDPOINT", &invalidate_endpoint)
        .replace("$URI", &uri)
        .replace("$ID", &id)
        .replace("$HTTP_METHOD", &http_method)
        .replace("$HEADERS", &headers);

    debug!("executing script");

    let result = execute_script(
        script.as_str(),
        if id == "\"inspection\"" {
            include_str!("./inspection.js")
        } else {
            &state.classes
        },
        state.cloudstate,
        state.blob_storage.clone(),
    )
    .await;

    debug!("script finished");

    let json = serde_json::from_str::<ScriptResponseResult>(&result).unwrap_or(
        ScriptResponseResult::Error {
            error: ErrorData {
                message: "Unknown error executing script".to_string(),
                stack: "Unknown error executing script".to_string(),
            },
        },
    );

    let mut builder = Response::builder();

    match json {
        ScriptResponseResult::Response { response } => {
            for (key, value) in response.headers {
                builder = builder.header(key, value);
            }
            let body = Body::from(response.bytes);
            builder.body(body).unwrap()
        }
        ScriptResponseResult::Error { error } => {
            let body = Body::from(error.message);
            builder
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap()
        }
    }
}

#[derive(Clone)]
struct AppState {
    cloudstate: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
    classes: String,
    env: HashMap<String, String>,
    invalidate_endpoint: String,
}

#[derive(Debug, Deserialize)]
struct MethodParams {
    params: Vec<serde_json::Value>,
}

async fn method_request(
    axum::extract::Path((id, method)): axum::extract::Path<(String, String)>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> axum::response::Json<serde_json::Value> {
    debug!("method_request");
    // turn into valid, sanitized, json string
    let id = serde_json::to_string(&id).unwrap();
    let method = serde_json::to_string(&method).unwrap();

    // get host from request
    let host = match request.headers().get("Host") {
        Some(h) => h.to_str().unwrap(),
        None => "www.example.com",
    };

    // TODO: find a way to not need the http:// prefix
    let uri = format!("https://{}{}", host, request.uri().path());
    let uri = serde_json::to_string(&uri).unwrap();

    let headers = request.headers();
    let headers = headers
        .iter()
        .map(|(key, value)| {
            format!(
                "{}: {}",
                serde_json::to_string(&key.to_string()).unwrap(),
                serde_json::to_string(&value.to_str().unwrap_or_default().to_string()).unwrap()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let headers = format!("{{{}}}", headers);

    let Json::<MethodParams>(params) = request.extract().await.unwrap();

    // only used for inspection api
    let run_script = &params.params.first().map(|p| p.as_str());

    let params = serde_json::to_string(&params.params).unwrap();
    let params = serde_json::to_string(&params).unwrap();
    let env_string = serde_json::to_string(&state.env).unwrap();
    let invalidate_endpoint = state.invalidate_endpoint.clone();

    // TODO: fix injection vulnerability
    let script = include_str!("./method_request.js")
        .replace("$ENV_STRING", &env_string)
        .replace("$URI", &uri)
        .replace("$HEADERS", &headers)
        .replace("$INVALIDATE_ENDPOINT", &invalidate_endpoint)
        .replace("$ID", &id)
        .replace("$METHOD", &method)
        .replace("$PARAMS", &params);

    debug!("executing script");
    let result = if id == "\"inspection\"" && method == "\"run\"" {
        let run_script = run_script.unwrap().unwrap();
        execute_script(
            &include_str!("./inspection_run.js")
                .replace("env_string", &env_string)
                .replace("run_script", run_script)
                .replace("invalidate_endpoint", &invalidate_endpoint),
            &state.classes,
            state.cloudstate,
            state.blob_storage.clone(),
        )
        .await
    } else {
        execute_script(
            script.as_str(),
            if id == "\"inspection\"" {
                include_str!("./inspection.js")
            } else {
                &state.classes
            },
            state.cloudstate,
            state.blob_storage.clone(),
        )
        .await
    };
    // debug!("script result: {:#?}", result);

    Json(serde_json::from_str(&result).unwrap_or(json!({
        "error": {
            "message": "Error executing script",
        }
    })))
}

struct CloudstateTimerPermissions {}

impl TimersPermission for CloudstateTimerPermissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

struct CloudstateFetchPermissions {}

impl FetchPermissions for CloudstateFetchPermissions {
    fn check_net_url(
        &mut self,
        _url: &url::Url,
        _api_name: &str,
    ) -> Result<(), PermissionCheckError> {
        debug!("checking net url fetch permission");
        Ok(())
    }

    fn check_read<'a>(
        &mut self,
        p: &'a Path,
        _api_name: &str,
    ) -> Result<std::borrow::Cow<'a, Path>, PermissionCheckError> {
        debug!("checking read fetch permission");
        Ok(p.to_path_buf().into())
    }
}

struct CloudstateNetPermissions {}

impl NetPermissions for CloudstateNetPermissions {
    fn check_net<T: AsRef<str>>(
        &mut self,
        _host: &(T, Option<u16>),
        _api_name: &str,
    ) -> Result<(), PermissionCheckError> {
        debug!("checking net permission");
        Ok(())
    }

    fn check_read(&mut self, p: &str, _api_name: &str) -> Result<PathBuf, PermissionCheckError> {
        debug!("checking read permission");
        Ok(p.to_string().into())
    }

    fn check_write(&mut self, p: &str, _api_name: &str) -> Result<PathBuf, PermissionCheckError> {
        debug!("checking write permission");
        Ok(p.to_string().into())
    }

    fn check_write_path<'a>(
        &mut self,
        p: &'a std::path::Path,
        _api_name: &str,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, PermissionCheckError> {
        debug!("checking write path permission");
        Ok(p.to_path_buf().into())
    }
}

pub async fn execute_script(
    script: &str,
    classes_script: &str,
    cs: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
) -> String {
    let script_string = script.to_string();
    let classes_script_string = classes_script.to_string();

    let span = tracing::info_span!("execute_script");

    tokio::task::spawn_blocking(move || {
        let _enter = span.enter();
        debug!("execute_script_internal blocking");
        execute_script_internal(&script_string, &classes_script_string, cs, blob_storage)
    })
    .await
    .unwrap()
}

// type CloudstateNodePermissions = AllowAllNodePermissions;

#[instrument(skip(script, classes_script, cs, blob_storage))]
#[tokio::main(flavor = "current_thread")]
pub async fn execute_script_internal(
    script: &str,
    classes_script: &str,
    cs: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
) -> String {
    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(CloudstateModuleLoader {
            lib: classes_script.to_string(),
        })),
        extensions: cloudstate_extensions(),
        extension_transpiler: Some(Rc::new(|specifier, source| {
            cloudstate_runtime::transpile::maybe_transpile_source(specifier, source)
        })),
        ..Default::default()
    });

    debug!("initializing runtime");

    RefCell::borrow_mut(&js_runtime.op_state()).put(cs.clone());
    RefCell::borrow_mut(&js_runtime.op_state()).put(CloudstatePermissions {});

    let main_module = ModuleSpecifier::from_file_path(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.js"),
    )
    .unwrap();

    RefCell::borrow_mut(&js_runtime.op_state()).put(CloudstateFetchPermissions {});
    let transaction_context = TransactionContext::new(cs.clone(), blob_storage.clone());
    RefCell::borrow_mut(&js_runtime.op_state()).put(transaction_context);
    RefCell::borrow_mut(&js_runtime.op_state()).put(JavaScriptSpans::new());
    // RefCell::borrow_mut(&js_runtime.op_state()).put(CloudstateNodePermissions {});

    let script = script.to_string();
    let future = async move {
        let mod_id = js_runtime
            .load_main_es_module_from_code(&main_module, script)
            .await
            .unwrap();

        debug!("evaluating module");
        let evaluation = js_runtime.mod_evaluate(mod_id);

        debug!("starting js event loop polling");
        let result = poll_fn(|cx| {
            let poll_result = js_runtime.poll_event_loop(cx, Default::default());
            let _ = js_runtime.execute_script("<handle>", "globalThis.commit();");
            poll_result
        })
        .await;
        debug!("ending js event loop polling");

        // let _ = js_runtime.execute_script("<handle>", "globalThis.commit();");

        let _ = evaluation.await;
        (js_runtime, result)
    };

    let (mut js_runtime, result) = future.await;
    event!(tracing::Level::DEBUG, "result: {:#?}", result);

    let mut js_runtime = js_runtime.handle_scope();
    let scope = js_runtime.borrow_mut();
    let context = scope.get_current_context();

    let global = context.global(scope);
    let key = v8_string_key!(scope, "result");
    let local_value = global.get(scope, key).unwrap();

    let json_value = v8::json::stringify(scope, local_value).unwrap_or(
        v8::String::new(
            scope,
            &json!({
                "error": {
                    "message": "Result could not be stringified",
                    "stack": "Result could not be stringified",
                }
            })
            .to_string(),
        )
        .unwrap(),
    );

    json_value.to_rust_string_lossy(scope)
}
