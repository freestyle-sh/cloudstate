use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use deno_core::{
    resolve_import, ModuleLoadResponse, ModuleLoader, ModuleSource, ModuleSourceCode,
    ModuleSpecifier, ModuleType, ResolutionKind,
};
use deno_web::TimersPermission;
// use serde::{Deserialize, Serialize};
use std::fs;

// struct CloudstateServer {
//     cloudstate: ReDBCloudstate,
//     router: Router,
// }

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
        let lib = fs::read_to_string("src/lib.js").unwrap();
        ModuleLoadResponse::Sync(Ok(ModuleSource::new(
            ModuleType::JavaScript,
            ModuleSourceCode::String(lib.into()),
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

// impl CloudstateServer {
//     pub fn new(cloudstate: ReDBCloudstate) -> Self {
//         // tracing_subscriber::fmt::init();

//         let shared_state = Arc::new(cloudstate);

//         let app = Router::new().route(
//             "/cloudstate/instances/:id/:method",
//             post({
//                 let shared_state = Arc::clone(&shared_state);
//                 move |body| {

//                 }
//             }),
//         );
//     }
// }

struct Permissions {}

impl TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, rc::Rc, sync::Arc};

    use cloudstate_runtime::{
        extensions::{bootstrap::bootstrap, cloudstate::cloudstate},
        print::print_database,
    };
    use deno_core::JsRuntime;
    use deno_web::BlobStore;
    use redb::backends::InMemoryBackend;

    use super::*;

    #[test]
    fn test() {
        let blob_storage = Arc::new(BlobStore::default());
        let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(CloudstateModuleLoader {
                lib: "src/lib.js".to_string(),
            })),
            // module_loader: None,
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

        let cs = ReDBCloudstate {
            db: redb::Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        };

        js_runtime.op_state().borrow_mut().put(cs);

        let main_module = ModuleSpecifier::from_file_path(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/server.js"),
        )
        .unwrap();

        let module_code = fs::read_to_string("src/server.js").unwrap();

        let future = async move {
            let mod_id = js_runtime
                .load_main_es_module_from_code(&main_module, module_code)
                .await
                .unwrap();

            let evaluation = js_runtime.mod_evaluate(mod_id);
            let result = js_runtime.run_event_loop(Default::default()).await;
            let _ = evaluation.await;
            (js_runtime, result)
        };

        let (mut js_runtime, result) = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(future);

        let cs = js_runtime.op_state().borrow_mut().take::<ReDBCloudstate>();

        print_database(&cs.db);

        result.unwrap();
    }
}
