use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Request, State},
    http::Response,
    routing::{get, post},
    Json, RequestExt, Router,
};
use cloudstate_runner::CloudstateRunner;
use cloudstate_runtime::{blob_storage::CloudstateBlobStorage, gc::mark_and_sweep, ServerInfo};
use deno_runtime::deno_permissions::PermissionCheckError;

use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use deno_core::*;
use deno_fetch::FetchPermissions;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, path::Path};
use tracing::{debug, instrument};

pub mod cloudstate_runner;
#[cfg(test)]
mod tests;

pub struct CloudstateServer<R: CloudstateRunner + 'static> {
    pub cloudstate: ReDBCloudstate,
    pub blob_storage: CloudstateBlobStorage,
    pub router: Router,
    pub cloudstate_runner: R,
    pub server_info: ServerInfo,
}

impl<R: CloudstateRunner> CloudstateServer<R> {
    pub async fn new(
        cloudstate: ReDBCloudstate,
        blob_storage: CloudstateBlobStorage,
        classes: &str,
        env: HashMap<String, String>,
        invalidate_endpoint: String,
        cloudstate_runner: R,
        server_info: ServerInfo,
    ) -> Self {
        let env_string = serde_json::to_string(&env).unwrap();
        cloudstate_runner
            .run_cloudstate(
                &include_str!("./initialize.js").replace("env_string", &env_string),
                classes,
                cloudstate.clone(),
                blob_storage.clone(),
                server_info.clone(),
            )
            .await;

        cloudstate_runner
            .run_cloudstate(
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
                server_info.clone(),
            )
            .await;

        let router = Router::new()
            .route(
                "/cloudstate/instances/{id}",
                get(fetch_request)
                    .post(fetch_request)
                    .patch(fetch_request)
                    .put(fetch_request)
                    .delete(fetch_request)
                    .head(fetch_request),
            )
            .route("/cloudstate/instances/{id}/{method}", post(method_request))
            .with_state(AppState {
                cloudstate: cloudstate.clone(),
                classes: classes.to_string(),
                env,
                invalidate_endpoint,
                blob_storage: blob_storage.clone(),
                cloudstate_runner: cloudstate_runner.clone(),
                server_info: server_info.clone(),
            });

        CloudstateServer {
            router,
            blob_storage,
            cloudstate,
            cloudstate_runner,
            server_info,
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

#[instrument(skip(id, state, request))]
async fn fetch_request<R: CloudstateRunner>(
    axum::extract::Path(id): axum::extract::Path<String>,
    State(state): State<AppState<R>>,
    request: Request,
) -> axum::response::Response {
    let id = serde_json::to_string(&id).unwrap();
    let (parts, body) = request.into_parts();
    let http_method = parts.method.to_string();

    let headers = parts.headers;
    let Some(Ok(host)) = headers.get("Host").map(|h| h.to_str()) else {
        return axum::response::Response::new(
            json!({
                "error": {
                    "message": "Host header is required",
                }
            })
            .to_string()
            .into(),
        );
    };
    let host = host.to_string();
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

    let result = state
        .cloudstate_runner
        .run_cloudstate(
            script.as_str(),
            if id == "\"inspection\"" {
                include_str!("./inspection.js")
            } else {
                &state.classes
            },
            state.cloudstate,
            state.blob_storage.clone(),
            state.server_info.clone(),
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
struct AppState<R: CloudstateRunner> {
    cloudstate: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
    classes: String,
    env: HashMap<String, String>,
    invalidate_endpoint: String,
    pub cloudstate_runner: R,
    server_info: ServerInfo,
}

#[derive(Debug, Deserialize)]
struct MethodParams {
    params: Vec<serde_json::Value>,
}

async fn method_request<R: CloudstateRunner>(
    axum::extract::Path((id, method)): axum::extract::Path<(String, String)>,
    State(state): State<AppState<R>>,
    request: Request<Body>,
) -> Json<serde_json::Value> {
    debug!("method_request");

    // Convert id, method to JSON strings
    let id = match serde_json::to_string(&id) {
        Ok(s) => s,
        Err(e) => {
            return Json(json!({ "error": { "message": format!("Error parsing id: {}", e) } }));
        }
    };
    let method = match serde_json::to_string(&method) {
        Ok(s) => s,
        Err(e) => {
            return Json(json!({ "error": { "message": format!("Error parsing method: {}", e) } }));
        }
    };

    // Extract Host header
    let Some(Ok(host)) = request.headers().get("Host").map(|h| h.to_str()) else {
        return Json(json!({ "error": { "message": "Host header is required" } }));
    };

    // Build full URI
    let uri_str = format!("https://{}{}", host, request.uri().path());
    let uri = match serde_json::to_string(&uri_str) {
        Ok(u) => u,
        Err(e) => {
            return Json(json!({ "error": { "message": format!("Error parsing uri: {}", e) } }));
        }
    };

    // Gather headers into JSON-friendly string
    let mut header_strings = vec![];
    for (key, value) in request.headers().iter() {
        let key_str = match serde_json::to_string(&key.to_string()) {
            Ok(k) => k,
            Err(e) => {
                return Json(
                    json!({ "error": { "message": format!("Error parsing header key: {}", e) } }),
                );
            }
        };
        let val_str = match value.to_str() {
            Ok(v) => match serde_json::to_string(&v.to_string()) {
                Ok(vv) => vv,
                Err(e) => {
                    return Json(
                        json!({ "error": { "message": format!("Error parsing header value: {}", e) } }),
                    );
                }
            },
            Err(e) => {
                return Json(
                    json!({ "error": { "message": format!("Invalid header value: {}", e) } }),
                );
            }
        };
        header_strings.push(format!("{}: {}", key_str, val_str));
    }
    let headers = format!("{{{}}}", header_strings.join(", "));

    // Extract body -> params
    let params = match request.extract::<Json<MethodParams>>().await {
        Ok(Json(params)) => params,
        Err(e) => {
            return Json(
                json!({ "error": { "message": format!("Error extracting params: {}", e) } }),
            );
        }
    };

    // Prepare params JSON
    let param_list = match serde_json::to_string(&params.params) {
        Ok(p) => p,
        Err(e) => {
            return Json(json!({ "error": { "message": format!("Error parsing params: {}", e) } }));
        }
    };
    let params_json = match serde_json::to_string(&param_list) {
        Ok(p) => p,
        Err(e) => {
            return Json(
                json!({ "error": { "message": format!("Error serializing params: {}", e) } }),
            );
        }
    };

    // Setup environment
    let env_string = match serde_json::to_string(&state.env) {
        Ok(s) => s,
        Err(e) => {
            return Json(
                json!({ "error": { "message": format!("Error serializing env: {}", e) } }),
            );
        }
    };
    let invalidate_endpoint = state.invalidate_endpoint.clone();

    // Inject into script
    let base_script = include_str!("./method_request.js")
        .replace("$ENV_STRING", &env_string)
        .replace("$URI", &uri)
        .replace("$HEADERS", &headers)
        .replace("$INVALIDATE_ENDPOINT", &invalidate_endpoint)
        .replace("$ID", &id)
        .replace("$METHOD", &method)
        .replace("$PARAMS", &params_json);

    debug!("executing script");

    // Special inspection case
    let run_result = if id == "\"inspection\"" && method == "\"run\"" {
        // We need the first param as the run script
        let Some(run_script_str) = params.params.first() else {
            return Json(json!({ "error": { "message": "No run script provided" } }));
        };
        let inspection_script = include_str!("./inspection_run.js")
            .replace("env_string", &env_string)
            .replace("run_script", run_script_str)
            .replace("invalidate_endpoint", &invalidate_endpoint);

        match state
            .cloudstate_runner
            .run_cloudstate(
                &inspection_script,
                &state.classes,
                state.cloudstate,
                state.blob_storage.clone(),
                state.server_info.clone(),
            )
            .await
        {
            Ok(res) => res,
            Err(e) => {
                return Json(
                    json!({ "error": { "message": format!("Error running inspection script: {}", e) } }),
                );
            }
        }
    } else {
        let classes = if id == "\"inspection\"" {
            include_str!("./inspection.js")
        } else {
            &state.classes
        };

        match state
            .cloudstate_runner
            .run_cloudstate(
                &base_script,
                classes,
                state.cloudstate,
                state.blob_storage.clone(),
                state.server_info.clone(),
            )
            .await
        {
            Ok(res) => res,
            Err(e) => {
                return Json(
                    json!({ "error": { "message": format!("Error running script: {}", e) } }),
                );
            }
        }
    };

    // Final JSON output
    match serde_json::from_str(&run_result) {
        Ok(json_output) => Json(json_output),
        Err(_) => Json(json!({ "error": { "message": "Error executing script" } })),
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
        resolved: bool,
        p: &'a Path,
        api_name: &str,
    ) -> Result<std::borrow::Cow<'a, Path>, deno_fs::FsError> {
        debug!("checking read fetch permission");
        Ok(p.to_path_buf().into())
    }
}
