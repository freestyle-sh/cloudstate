use deno_core::*;
use deno_fetch::FetchPermissions;
use deno_net::NetPermissions;
use deno_web::{BlobStore, TimersPermission};
use futures::future::poll_fn;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use tracing::event;

use crate::extensions::bootstrap::bootstrap;
use crate::extensions::cloudstate::{cloudstate, ReDBCloudstate};

struct Permissions {}

impl TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

struct CloudstateFetchPermissions {}

impl FetchPermissions for CloudstateFetchPermissions {
    fn check_net_url(&mut self, _url: &url::Url, _api_name: &str) -> Result<(), error::AnyError> {
        event!(tracing::Level::DEBUG, "checking net url fetch permission");
        Ok(())
    }
    fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<(), error::AnyError> {
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
    ) -> Result<(), error::AnyError> {
        event!(tracing::Level::DEBUG, "checking net permission");
        Ok(())
    }
    fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<(), error::AnyError> {
        event!(tracing::Level::DEBUG, "checking read permission");
        Ok(())
    }
    fn check_write(&mut self, _p: &Path, _api_name: &str) -> Result<(), error::AnyError> {
        event!(tracing::Level::DEBUG, "checking write permission");
        Ok(())
    }
}

pub fn run_script(
    path: &str,
    cloudstate: ReDBCloudstate,
) -> Result<
    (
        Arc<std::sync::Mutex<ReDBCloudstate>>,
        Result<(), anyhow::Error>,
    ),
    anyhow::Error,
> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let blob_storage = Arc::new(BlobStore::default());
    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            deno_web::deno_web::init_ops_and_esm::<Permissions>(blob_storage, None),
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

    js_runtime
        .op_state()
        .borrow_mut()
        .put(Arc::new(std::sync::Mutex::new(cloudstate)));
    js_runtime
        .op_state()
        .borrow_mut()
        .put(CloudstateFetchPermissions {});

    let future = async move {
        let mod_id = js_runtime.load_main_es_module(&main_module).await.unwrap();
        let evaluation = js_runtime.mod_evaluate(mod_id);
        // let result = js_runtime.run_event_loop(Default::default()).await;

        let result = poll_fn(|cx| {
            event!(tracing::Level::DEBUG, "committing");
            let _ = js_runtime.execute_script("<handle>", "globalThis.commit();");
            event!(tracing::Level::DEBUG, "polling event loop");
            js_runtime.poll_event_loop(
                cx,
                PollEventLoopOptions {
                    pump_v8_message_loop: true,
                    wait_for_inspector: false,
                },
            )
        })
        .await;

        let _ = evaluation.await;

        (js_runtime, result)
    };

    let (mut js_runtime, result) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    let cloudstate = js_runtime
        .op_state()
        .borrow_mut()
        .take::<Arc<std::sync::Mutex<ReDBCloudstate>>>();

    return Ok((cloudstate, result));
}
