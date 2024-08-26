use axum::{
    extract::{Extension, Path, State},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use deno_core::{
    futures::future::poll_fn, resolve_import, ModuleLoadResponse, ModuleLoader, ModuleSource,
    ModuleSourceCode, ModuleSpecifier, ModuleType, ResolutionKind,
};
use deno_web::TimersPermission;
use redb::Database;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, rc::Rc, sync::Arc};
use tokio::sync::RwLock;

use cloudstate_runtime::{
    extensions::{bootstrap::bootstrap, cloudstate::cloudstate},
    print::print_database,
};
use deno_core::JsRuntime;
use deno_web::BlobStore;
use redb::backends::InMemoryBackend;

pub struct CloudstateServer {
    cloudstate: Arc<ReDBCloudstate>,
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
        kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, anyhow::Error> {
        Ok(resolve_import(specifier, referrer)?)
    }
}

impl CloudstateServer {
    pub async fn new(cloudstate: ReDBCloudstate, classes: &str) -> Self {
        // tracing_subscriber::fmt::init();

        let cloudstate = Arc::new(cloudstate);

        execute_script(include_str!("./initialize.js"), classes, cloudstate.clone()).await;

        let app = Router::new()
            .route("/cloudstate/instances/:id/:method", post(method_request))
            .with_state(AppState {
                cloudstate: cloudstate.clone(),
                classes: classes.to_string(),
            });

        CloudstateServer {
            cloudstate: cloudstate,
            router: app,
        }
    }
}

#[derive(Clone)]
struct AppState {
    cloudstate: Arc<RwLock<ReDBCloudstate>>,
    classes: String,
}

#[derive(Debug, Deserialize)]
struct MethodParams {
    params: Vec<serde_json::Value>,
}

async fn method_request(
    Path(id): Path<String>,
    Path(method): Path<String>,
    State(state): State<AppState>,
    Json(params): Json<MethodParams>,
) -> axum::response::Html<String> {
    let _ = execute_script(
        // todo: fix injection vulnerability
        format!(
            "
        import * as classes from './lib.js';
        const cloudstate = new Cloudstate('default', {{
            customClasses: Object.keys(classes).map((key) => classes[key]),
        }});
    
        const transaction = cloudstate.createTransaction();
        const object =transaction.getRoot('{id}');
        object.{method}();
        transaction.commit();
    ",
        )
        .as_str(),
        &state.classes,
        state.cloudstate,
    )
    .await;

    Html("<html><body>test</body></html>".to_string())
}

struct Permissions {}

impl TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

pub async fn execute_script(script: &str, classes_script: &str, cs: Arc<RwLock<ReDBCloudstate>>) {
    let script_string = script.to_string();
    let classes_script_string = classes_script.to_string();
    tokio::task::spawn_blocking(move || {
        execute_script_internal(&script_string, &classes_script_string, cs);
    })
    .await
    .unwrap();
}

#[tokio::main]
pub async fn execute_script_internal(
    script: &str,
    classes_script: &str,
    cs: Arc<RwLock<ReDBCloudstate>>,
) {
    let blob_storage = Arc::new(BlobStore::default());
    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(CloudstateModuleLoader {
            lib: classes_script.to_string(),
        })),
        extensions: vec![
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            deno_web::deno_web::init_ops_and_esm::<Permissions>(blob_storage, None),
            deno_crypto::deno_crypto::init_ops_and_esm(None),
            bootstrap::init_ops_and_esm(),
            cloudstate::init_ops_and_esm(),
        ],
        ..Default::default()
    });

    js_runtime.op_state().borrow_mut().put(cs);

    let main_module = ModuleSpecifier::from_file_path(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.js"),
    )
    .unwrap();

    let script = script.to_string();
    let future = async move {
        let mod_id = js_runtime
            .load_main_es_module_from_code(&main_module, script)
            .await
            .unwrap();

        let evaluation = js_runtime.mod_evaluate(mod_id);
        // let result = js_runtime.run_event_loop(Default::default()).await;

        let result = poll_fn(|cx| self.poll_event_loop(cx, poll_options)).await;

        let _ = evaluation.await;
        (js_runtime, result)
    };

    // let (mut js_runtime, result) = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(future);

    let (mut js_runtime, result) = future.await;

    let cs = js_runtime
        .op_state()
        .borrow_mut()
        .take::<Arc<ReDBCloudstate>>();

    print_database(&cs.db);

    result.unwrap();
}
