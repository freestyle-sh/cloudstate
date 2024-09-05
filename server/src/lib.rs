use axum::{
    body::Body,
    extract::{Host, Path, Request, State},
    http::Response,
    routing::{get, post},
    Json, RequestExt, Router,
};
use cloudstate_runtime::extensions::{bootstrap::bootstrap, cloudstate::cloudstate};
use cloudstate_runtime::{extensions::cloudstate::ReDBCloudstate, v8_string_key};
use deno_core::*;
use deno_core::{
    futures::future::poll_fn, resolve_import, ModuleLoadResponse, ModuleLoader, ModuleSource,
    ModuleSourceCode, ModuleSpecifier, ModuleType, ResolutionKind,
};
use deno_core::{url::Url, JsRuntime};
use deno_fetch::FetchPermissions;
use deno_net::NetPermissions;
use deno_web::BlobStore;
use deno_web::TimersPermission;
use futures::TryStreamExt;
use serde::Deserialize;
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, sync::Mutex};
use std::{rc::Rc, sync::Arc};
use tracing::{debug, event};

#[cfg(test)]
mod tests;

pub struct CloudstateServer {
    // pub cloudstate: Arc<Mutex<ReDBCloudstate>>,
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
        cloudstate: Arc<Mutex<ReDBCloudstate>>,
        classes: &str,
        env: HashMap<String, String>,
    ) -> Self {
        // tracing_subscriber::fmt::init();

        execute_script(include_str!("./initialize.js"), classes, cloudstate.clone()).await;

        let app = Router::new()
            .route(
                "/cloudstate/instances/:id",
                get(fetch_request)
                    .post(fetch_request)
                    .patch(fetch_request)
                    .put(fetch_request)
                    .delete(fetch_request),
            )
            .route("/cloudstate/instances/:id/:method", post(method_request))
            .with_state(AppState {
                cloudstate: cloudstate.clone(),
                classes: classes.to_string(),
                env: env,
            });

        CloudstateServer { router: app }
    }
}

#[derive(Deserialize, Debug)]
struct ScriptResponseResult {
    pub result: ResponseData,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    pub bytes: Vec<u8>,
    pub headers: Vec<(String, String)>,
}

async fn fetch_request(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Host(host): Host,
    request: Request,
) -> axum::response::Response {
    let id = serde_json::to_string(&id).unwrap();
    let (parts, body) = request.into_parts();

    let method = serde_json::to_string(&parts.method.to_string()).unwrap();
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

    let bytes = format!(
        "[{}]",
        bytes
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    // // body: new Uint8Array({bytes}).buffer,
    // // headers: new Headers({headers}),
    // let headers = headers.to_string();

    //headers: new Headers({headers}),
    // method: '{method}',
    //{url},
    // TODO: fix injection vulnerability
    let script = format!(
        "
    import * as classes from './lib.js';
    globalThis.cloudstate.customClasses = Object.keys(classes).map((key) => classes[key]);

    let bytes = new Uint8Array({bytes});

    const method =  {method};


    const object = getRoot({id});
    try {{
        const req = new Request({uri},
            {{
                method,
                headers: new Headers({headers}),
                body: ['GET', 'HEAD'].includes(method) ? undefined : bytes.buffer
            }}
        );

        let out = object['fetch'](req);

        if (out instanceof Promise) {{
            console.log('fetch is a promise');
            out = await out;
        }}

        if (out instanceof Response) {{
            const body = await out.bytes();
            const headers = [...out.headers.entries()];

            // uint8array to array
            let bytes = Array.from(body);



            globalThis.result = {{ result: {{ bytes, headers }} }};
        }}

    }} catch (e) {{
        globalThis.result = {{ error: {{ message: e.message, stack: e.stack }} }};
    }}
    ",
    );

    event!(tracing::Level::DEBUG, "executing script");

    let result = execute_script(&script.as_str(), &state.classes, state.cloudstate).await;

    event!(tracing::Level::DEBUG, "script finished");

    let json = serde_json::from_str::<ScriptResponseResult>(&result).unwrap();

    event!(tracing::Level::DEBUG, "json: {:#?}", json);

    let mut builder = Response::builder();
    for (key, value) in json.result.headers {
        builder = builder.header(key, value);
    }

    let body = Body::from(json.result.bytes);
    let builder = builder.body(body).unwrap();

    builder
}

#[derive(Clone)]
struct AppState {
    cloudstate: Arc<Mutex<ReDBCloudstate>>,
    classes: String,
    env: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct MethodParams {
    params: Vec<serde_json::Value>,
}

async fn method_request(
    Path((id, method)): Path<(String, String)>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> axum::response::Json<serde_json::Value> {
    debug!("method_request");
    // turn into valid, sanitized, json string
    let id = serde_json::to_string(&id).unwrap();
    let method = serde_json::to_string(&method).unwrap();

    // get host from request
    let host = request.headers().get("Host").unwrap().to_str().unwrap();

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
    let params = serde_json::to_string(&params.params).unwrap();

    let env_string = serde_json::to_string(&state.env).unwrap();

    // TODO: fix injection vulnerability
    let script = format!(
        "
    import * as classes from './lib.js';
    globalThis.cloudstate.customClasses = Object.keys(classes).map((key) => classes[key]);

    globalThis.process = {{
        env: {env_string}
    }}

    // temporary hack to be compatible with legacy freestyle apis
    globalThis.requestContext = {{
        getStore: () => {{
            return {{
                request: new Request({uri}, {{
                    headers: new Headers({headers}),
                }}),
                env: {{
                    invalidateMethod: (rawMethod) => {{
                        const method = rawMethod.toJSON();
                        fetch(`http://localhost:8910/__invalidate__/${{method.instance}}/${{method.method}}`, {{
                            method: 'POST',
                            headers: {{
                                'Content-Type': 'application/json',
                            }}
                        }}).catch(e => {{
                            console.error(e);
                        }});
                    }},
                }}
            }}
        }}
    }}

    const object = getRoot({id}) || getCloudstate({id});
    try {{
       globalThis.result = {{ result: await object[{method}](...JSON.parse('{params}')) }};
    }} catch (e) {{
        globalThis.result = {{ error: {{ message: e.message, stack: e.stack }} }};
    }}
    ",
    );

    event!(tracing::Level::DEBUG, "executing script");
    let result = execute_script(&script.as_str(), &state.classes, state.cloudstate).await;
    event!(tracing::Level::DEBUG, "script result: {:#?}", result);

    Json(serde_json::from_str(&result).unwrap())
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
        _url: &Url,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        event!(tracing::Level::DEBUG, "checking net url fetch permission");
        Ok(())
    }
    fn check_read(
        &mut self,
        _p: &std::path::Path,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        event!(tracing::Level::DEBUG, "checking read fetch permission");
        Ok(())
    }
}

struct CloudstateNetPermissions {}

impl NetPermissions for CloudstateNetPermissions {
    fn check_net<T: AsRef<str>>(
        &mut self,
        _host: &(T, Option<u16>),
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        event!(tracing::Level::DEBUG, "checking net permission");
        Ok(())
    }
    fn check_read(
        &mut self,
        _p: &std::path::Path,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        event!(tracing::Level::DEBUG, "checking read permission");
        Ok(())
    }
    fn check_write(
        &mut self,
        _p: &std::path::Path,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        event!(tracing::Level::DEBUG, "checking write permission");
        Ok(())
    }
}

pub async fn execute_script(
    script: &str,
    classes_script: &str,
    cs: Arc<Mutex<ReDBCloudstate>>,
) -> String {
    let script_string = script.to_string();
    let classes_script_string = classes_script.to_string();

    tokio::task::spawn_blocking(move || {
        debug!("execute_script_internal blocking");
        execute_script_internal(&script_string, &classes_script_string, cs)
    })
    .await
    .unwrap()
}

#[tokio::main(flavor = "current_thread")]
pub async fn execute_script_internal(
    script: &str,
    classes_script: &str,
    cs: Arc<Mutex<ReDBCloudstate>>,
) -> String {
    let blob_storage = Arc::new(BlobStore::default());
    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(CloudstateModuleLoader {
            lib: classes_script.to_string(),
        })),
        extensions: vec![
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            deno_web::deno_web::init_ops_and_esm::<CloudstateTimerPermissions>(blob_storage, None),
            deno_crypto::deno_crypto::init_ops_and_esm(None),
            bootstrap::init_ops_and_esm(),
            deno_fetch::deno_fetch::init_ops_and_esm::<CloudstateFetchPermissions>(
                Default::default(),
            ),
            deno_net::deno_net::init_ops_and_esm::<CloudstateNetPermissions>(None, None),
            cloudstate::init_ops_and_esm(),
        ],
        ..Default::default()
    });
    debug!("initializing runtime");

    RefCell::borrow_mut(&js_runtime.op_state()).put(cs);

    let main_module = ModuleSpecifier::from_file_path(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.js"),
    )
    .unwrap();

    RefCell::borrow_mut(&js_runtime.op_state()).put(CloudstateFetchPermissions {});

    let script = script.to_string();
    let future = async move {
        let mod_id = js_runtime
            .load_main_es_module_from_code(&main_module, script)
            .await
            .unwrap();

        let evaluation = js_runtime.mod_evaluate(mod_id);
        // let result = js_runtime.run_event_loop(Default::default()).await;

        event!(tracing::Level::DEBUG, "starting polling");
        let result = poll_fn(|cx| {
            let _ = js_runtime.execute_script("<handle>", "globalThis.commit();");
            js_runtime.poll_event_loop(cx, Default::default())
        })
        .await;

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

    let json_value = v8::json::stringify(scope, local_value).unwrap();
    let json_str = json_value.to_rust_string_lossy(scope);
    json_str
}
