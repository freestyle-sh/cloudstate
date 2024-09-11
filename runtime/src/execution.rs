use deno_core::*;
use deno_fetch::FetchPermissions;
use deno_fs::InMemoryFs;
use deno_net::NetPermissions;
use deno_node::AllowAllNodePermissions;
use deno_web::{BlobStore, TimersPermission};
use futures::future::poll_fn;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use tracing::event;

use crate::extensions::bootstrap::bootstrap;
use crate::extensions::cloudstate::{cloudstate, ReDBCloudstate, TransactionContext};

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
) -> Result<(ReDBCloudstate, Result<(), anyhow::Error>), anyhow::Error> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);

    let blob_storage = Arc::new(BlobStore::default());

    let mut result = Err(anyhow::anyhow!("No files found"));
    for (i, source) in fs::read_to_string(js_path.clone())
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

type CloudstateNodePermissions = AllowAllNodePermissions;

pub fn run_script_source(
    script: &str,
    cloudstate: ReDBCloudstate,
    blob_storage: Arc<BlobStore>,
    path: PathBuf,
) -> Result<(ReDBCloudstate, Result<(), anyhow::Error>), anyhow::Error> {
    let main_module = ModuleSpecifier::from_file_path(path).unwrap();
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
            // deno_node::deno_node::init_ops_and_esm::<CloudstateNodePermissions>(
            //     None,
            //     std::rc::Rc::new(InMemoryFs::default()),
            // ),
        ],
        ..Default::default()
    });

    let transaction_context = TransactionContext::new(cloudstate.clone());
    js_runtime.op_state().borrow_mut().put(transaction_context);
    js_runtime
        .op_state()
        .borrow_mut()
        .put(CloudstateFetchPermissions {});

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

        let _ = evaluation.await;

        (js_runtime, result)
    };

    let (_, result) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    return Ok((cloudstate, result));
}
