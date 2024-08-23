use deno_core::*;
use std::path::Path;
use std::rc::Rc;

use crate::extensions::bootstrap::bootstrap;
use crate::extensions::cloudstate::{cloudstate, ReDBCloudstate};

pub fn run_script(
    path: &str,
    cloudstate: ReDBCloudstate,
) -> Result<(ReDBCloudstate, Result<(), anyhow::Error>), anyhow::Error> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            bootstrap::init_ops_and_esm(),
            cloudstate::init_ops_and_esm(),
        ],
        ..Default::default()
    });

    js_runtime.op_state().borrow_mut().put(cloudstate);

    let future = async move {
        let mod_id = js_runtime.load_main_es_module(&main_module).await.unwrap();
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

    let cloudstate = js_runtime.op_state().borrow_mut().take::<ReDBCloudstate>();

    return Ok((cloudstate, result));
}
