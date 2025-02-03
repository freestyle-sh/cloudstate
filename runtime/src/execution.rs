use deno_core::*;
// use deno_runtime::ops::runtime::deno_runtime;
// use deno_node::AllowAllNodePermissions;
use futures::future::poll_fn;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::event;

use crate::blob_storage::CloudstateBlobStorage;
use crate::cloudstate_extensions::cloudstate_extensions;
use crate::extensions::cloudstate::{JavaScriptSpans, ReDBCloudstate, TransactionContext};
use crate::permissions::CloudstatePermissions;
use crate::{transpile, ServerInfo};

pub fn run_script(
    path: &str,
    cloudstate: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
) -> Result<(ReDBCloudstate, Result<(), anyhow::Error>), anyhow::Error> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);

    let mut result = Err(anyhow::anyhow!("No files found"));
    for (_i, source) in fs::read_to_string(js_path.clone())
        .unwrap()
        .split("// END_FILE")
        .enumerate()
    {
        result = Ok(run_script_source(
            source,
            cloudstate.clone(),
            blob_storage.clone(),
            js_path.clone(),
        )?);
    }

    result
}

// type CloudstateNodePermissions = AllowAllNodePermissions;

pub fn run_script_source(
    script: &str,
    cloudstate: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
    path: PathBuf,
) -> Result<(ReDBCloudstate, Result<(), anyhow::Error>), anyhow::Error> {
    let main_module = ModuleSpecifier::from_file_path(path).unwrap();

    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: cloudstate_extensions(),
        extension_transpiler: Some(Rc::new(|specifier, source| {
            transpile::maybe_transpile_source(specifier, source)
        })),
        ..Default::default()
    });

    let transaction_context = TransactionContext::new(cloudstate.clone(), blob_storage.clone());
    js_runtime.op_state().borrow_mut().put(transaction_context);
    js_runtime
        .op_state()
        .borrow_mut()
        .put(CloudstatePermissions {});
    js_runtime
        .op_state()
        .borrow_mut()
        .put(JavaScriptSpans::new());
    js_runtime.op_state().borrow_mut().put(ServerInfo {
        deployment_id: None,
    });

    let script = script.to_string();
    let future = async move {
        let mod_id = js_runtime
            .load_main_es_module_from_code(&main_module, script.clone())
            .await
            .unwrap();
        let evaluation = js_runtime.mod_evaluate(mod_id);

        let result = poll_fn(|cx| {
            event!(tracing::Level::DEBUG, "polling event loop");
            let poll_result = js_runtime.poll_event_loop(
                cx,
                PollEventLoopOptions {
                    pump_v8_message_loop: true,
                    wait_for_inspector: false,
                },
            );

            js_runtime
                .op_state()
                .borrow_mut()
                .borrow_mut::<TransactionContext>()
                .commit_transaction();

            poll_result
        })
        .await;

        js_runtime
            .op_state()
            .borrow_mut()
            .borrow_mut::<TransactionContext>()
            .commit_transaction();

        let _ = evaluation.await;

        (js_runtime, result)
    };

    let (_, result) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    let result = result.map_err(|err| anyhow::anyhow!(err));

    Ok((cloudstate, result))
}
