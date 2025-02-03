use std::{cell::RefCell, future::poll_fn, rc::Rc, task::Poll};

use cloudstate_runtime::{
    blob_storage::CloudstateBlobStorage,
    cloudstate_extensions::cloudstate_extensions,
    extensions::cloudstate::{JavaScriptSpans, ReDBCloudstate, TransactionContext},
    permissions::CloudstatePermissions,
    v8_string_key,
};
use deno_core::{v8, JsRuntime, ModuleSpecifier};
use deno_runtime::js;
use futures_util::FutureExt;
use serde_json::json;
use tracing::{debug, event, instrument};

use crate::{
    cloudstate_runner::module_loader::CloudstateModuleLoader, CloudstateFetchPermissions,
    ServerInfo,
};

pub async fn execute_script(
    script: &str,
    classes_script: &str,
    cs: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
    server_info: crate::ServerInfo,
    // js_runner: impl CloudstateRunner + 'static,
) -> String {
    let script_string = script.to_string();
    let classes_script_string = classes_script.to_string();

    let span = tracing::info_span!("execute_script");

    tokio::task::spawn_blocking(move || {
        let _enter = span.enter();
        debug!("execute_script_internal blocking");
        execute_script_internal(
            &script_string,
            &classes_script_string,
            cs,
            blob_storage,
            server_info,
        )
    })
    .await
    .unwrap()
}

// type CloudstateNodePermissions = AllowAllNodePermissions;

#[instrument(skip(script, classes_script, cs, blob_storage, server_info))]
#[tokio::main(flavor = "current_thread")]
pub async fn execute_script_internal(
    script: &str,
    classes_script: &str,
    cs: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
    server_info: crate::ServerInfo,
) -> String {
    let (sender, reciever) = tokio::sync::oneshot::channel();
    let js_runtime = initialize_cloudstate_runtime(reciever, server_info.clone());

    run_script(script, classes_script, cs, blob_storage, js_runtime, sender).await
}

pub fn initialize_cloudstate_runtime(
    reciever: tokio::sync::oneshot::Receiver<String>,
    server_info: ServerInfo,
) -> JsRuntime {
    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(CloudstateModuleLoader::new_async(reciever))),
        extensions: cloudstate_extensions(),
        extension_transpiler: Some(Rc::new(|specifier, source| {
            cloudstate_runtime::transpile::maybe_transpile_source(specifier, source)
        })),
        ..Default::default()
    });

    debug!("initializing runtime");

    RefCell::borrow_mut(&js_runtime.op_state()).put(CloudstatePermissions {});
    RefCell::borrow_mut(&js_runtime.op_state()).put(CloudstateFetchPermissions {});
    RefCell::borrow_mut(&js_runtime.op_state()).put(JavaScriptSpans::new());
    RefCell::borrow_mut(&js_runtime.op_state()).put(server_info);

    // RefCell::borrow_mut(&js_runtime.op_state()).put(CloudstateNodePermissions {});

    js_runtime
}

pub async fn run_script(
    script: &str,
    classes_script: &str,
    cs: ReDBCloudstate,
    blob_storage: CloudstateBlobStorage,
    mut js_runtime: JsRuntime,
    sender: tokio::sync::oneshot::Sender<String>,
) -> String {
    sender
        .send(classes_script.to_string())
        .expect("failed to send classes script");
    let transaction_context = TransactionContext::new(cs.clone(), blob_storage.clone());
    RefCell::borrow_mut(&js_runtime.op_state()).put(cs.clone());

    RefCell::borrow_mut(&js_runtime.op_state()).put(transaction_context);

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

        debug!("evaluating module");
        let mut evaluation = js_runtime.mod_evaluate(mod_id);

        debug!("starting js event loop polling");
        let result = poll_fn(|cx| match evaluation.poll_unpin(cx) {
            Poll::Pending => {
                let poll_result = js_runtime.poll_event_loop(cx, Default::default());
                let _ = js_runtime.execute_script("<handle>", "globalThis.commit();");
                poll_result
            }
            Poll::Ready(result) => return Poll::Ready(result),
        })
        .await;
        debug!("ending js event loop polling");

        (js_runtime, result)
    };

    let (mut js_runtime, result) = future.await;
    event!(tracing::Level::DEBUG, "result: {:#?}", result);

    let mut js_runtime = js_runtime.handle_scope();
    let scope = &mut js_runtime;
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
