use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
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
use serde::Deserialize;
use std::{borrow::BorrowMut, cell::RefCell, sync::Mutex};
use std::{rc::Rc, sync::Arc};

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
    pub async fn new(cloudstate: Arc<Mutex<ReDBCloudstate>>, classes: &str) -> Self {
        // tracing_subscriber::fmt::init();

        execute_script(include_str!("./initialize.js"), classes, cloudstate.clone()).await;

        let app = Router::new()
            .route("/cloudstate/instances/:id/:method", post(method_request))
            .with_state(AppState {
                cloudstate: cloudstate.clone(),
                classes: classes.to_string(),
            });

        CloudstateServer { router: app }
    }
}

#[derive(Clone)]
struct AppState {
    cloudstate: Arc<Mutex<ReDBCloudstate>>,
    classes: String,
}

#[derive(Debug, Deserialize)]
struct MethodParams {
    params: Vec<serde_json::Value>,
}

async fn method_request(
    Path((id, method)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(params): Json<MethodParams>,
) -> axum::response::Json<serde_json::Value> {
    // turn into valid, sanitized, json string
    let id = serde_json::to_string(&id).unwrap();
    let method = serde_json::to_string(&method).unwrap();
    println!("id: {:?}", id);
    println!("method: {:?}", method);
    let params = serde_json::to_string(&params.params).unwrap();
    println!("params: {:?}", params);

    // todo: fix injection vulnerability
    let script = format!(
        "
    import * as classes from './lib.js';
    globalThis.cloudstate.customClasses = Object.keys(classes).map((key) => classes[key]);

    const object = getRoot({id});
    try {{
        globalThis.result = {{result: object[{method}](...JSON.parse('{params}'))}};
    }} catch (e) {{
        globalThis.result = {{ error: {{ message: e.message, stack: e.stack }} }};
    }}
    ",
    );

    println!("executing script: {:#?}", script);

    let result = execute_script(&script.as_str(), &state.classes, state.cloudstate).await;

    println!("completed script");
    println!("result: {:?}", result);

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
        println!("checking net url fetch permission");
        Ok(())
    }
    fn check_read(
        &mut self,
        _p: &std::path::Path,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        println!("checking read fetch permission");
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
        println!("checking net");
        Ok(())
    }
    fn check_read(
        &mut self,
        _p: &std::path::Path,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        println!("checking read");
        Ok(())
    }
    fn check_write(
        &mut self,
        _p: &std::path::Path,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        println!("checking write");
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
        execute_script_internal(&script_string, &classes_script_string, cs)
    })
    .await
    .unwrap()
}

#[tokio::main]
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
    println!("initialized js runtime");

    RefCell::borrow_mut(&js_runtime.op_state()).put(cs);

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

        println!("starting polling");
        let result = poll_fn(|cx| {
            let _ = js_runtime.execute_script("<handle>", "globalThis.commit();");
            js_runtime.poll_event_loop(cx, Default::default())
        })
        .await;

        let _ = evaluation.await;
        (js_runtime, result)
    };

    let (mut js_runtime, result) = future.await;
    println!("completed polling");

    let cs = RefCell::borrow_mut(&js_runtime.op_state()).take::<Arc<Mutex<ReDBCloudstate>>>();

    // let cs = &cs.lock().unwrap();
    // print_database(&cs.db);

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
