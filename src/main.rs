mod extensions;

use deno_core::anyhow::Error;
use deno_core::*;
use std::path::Path;
use std::rc::Rc;

use extensions::bootstrap::bootstrap;
use extensions::cloudstate::cloudstate;
use extensions::superjson::superjson;

fn main() -> Result<(), Error> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/main.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            bootstrap::init_ops_and_esm(),
            superjson::init_ops_and_esm(),
            cloudstate::init_ops_and_esm(),
        ],
        ..Default::default()
    });

    let future = async move {
        let mod_id = js_runtime.load_main_es_module(&main_module).await?;
        let result = js_runtime.mod_evaluate(mod_id);
        js_runtime.run_event_loop(Default::default()).await?;
        result.await
    };

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}
