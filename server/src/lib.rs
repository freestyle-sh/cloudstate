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
) -> axum::response::Json<serde_json::Value> {
    debug!("method_request");
    // turn into valid, sanitized, json string
    let id = serde_json::to_string(&id).unwrap();
    let method = serde_json::to_string(&method).unwrap();

    // get host from request
    let Some(Ok(host)) = request.headers().get("Host").map(|h| h.to_str()) else {
        return Json(json!({
            "error": {
                "message": "Host header is required",
            }
        }));
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

        state
            .cloudstate_runner
            .run_cloudstate(
                &include_str!("./inspection_run.js")
                    .replace("env_string", &env_string)
                    .replace("run_script", run_script)
                    .replace("invalidate_endpoint", &invalidate_endpoint),
                &state.classes,
                state.cloudstate,
                state.blob_storage.clone(),
                state.server_info.clone(),
            )
            .await
    } else {
        state
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
            .await
    };

    Json(serde_json::from_str(&result).unwrap_or(json!({
        "error": {
            "message": "Error executing script",
        }
    })))
}

// struct CloudstateTimerPermissions {}

// impl TimersPermission for CloudstateTimerPermissions {
//     fn allow_hrtime(&mut self) -> bool {
//         false
//     }
// }

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

// struct CloudstateNetPermissions {}

// impl NetPermissions for CloudstateNetPermissions {
//     fn check_net<T: AsRef<str>>(
//         &mut self,
//         _host: &(T, Option<u16>),
//         _api_name: &str,
//     ) -> Result<(), PermissionCheckError> {
//         debug!("checking net permission");
//         Ok(())
//     }

//     fn check_read(&mut self, p: &str, _api_name: &str) -> Result<PathBuf, PermissionCheckError> {
//         debug!("checking read permission");
//         Ok(p.to_string().into())
//     }

//     fn check_write(&mut self, p: &str, _api_name: &str) -> Result<PathBuf, PermissionCheckError> {
//         debug!("checking write permission");
//         Ok(p.to_string().into())
//     }

//     fn check_write_path<'a>(
//         &mut self,
//         p: &'a std::path::Path,
//         _api_name: &str,
//     ) -> Result<std::borrow::Cow<'a, std::path::Path>, PermissionCheckError> {
//         debug!("checking write path permission");
//         Ok(p.to_path_buf().into())
//     }
// }
