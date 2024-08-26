use deno_core::*;
use deno_fetch::FetchPermissions;
use deno_net::NetPermissions;
use deno_web::{BlobStore, TimersPermission};
use futures::future::poll_fn;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use crate::extensions::bootstrap::bootstrap;
use crate::extensions::cloudstate::{cloudstate, ReDBCloudstate};
use crate::print;

struct Permissions {}

impl TimersPermission for Permissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

struct CloudstateFetchPermissions {}

impl FetchPermissions for CloudstateFetchPermissions {
    fn check_net_url(&mut self, _url: &url::Url, api_name: &str) -> Result<(), error::AnyError> {
        println!("checking net url fetch permission");
        Ok(())
    }
    fn check_read(&mut self, _p: &Path, api_name: &str) -> Result<(), error::AnyError> {
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
    ) -> Result<(), error::AnyError> {
        println!("checking net");
        Ok(())
    }
    fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<(), error::AnyError> {
        println!("checking read");
        Ok(())
    }
    fn check_write(&mut self, _p: &Path, _api_name: &str) -> Result<(), error::AnyError> {
        println!("checking write");
        Ok(())
    }
}

pub fn run_script(
    path: &str,
    cloudstate: ReDBCloudstate,
) -> Result<(ReDBCloudstate, Result<(), anyhow::Error>), anyhow::Error> {
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
            cloudstate::init_ops_and_esm(),
            deno_fetch::deno_fetch::init_ops_and_esm::<CloudstateFetchPermissions>(
                Default::default(),
            ),
            deno_net::deno_net::init_ops_and_esm::<CloudstateNetPermissions>(None, None),
        ],
        ..Default::default()
    });

    js_runtime.op_state().borrow_mut().put(cloudstate);

    let future = async move {
        let mod_id = js_runtime.load_main_es_module(&main_module).await.unwrap();
        let evaluation = js_runtime.mod_evaluate(mod_id);
        // let result = js_runtime.run_event_loop(Default::default()).await;

        let result = poll_fn(|cx| {
            // let context = js_runtime.handle_scope();
            // let mut scope = js_runtime.handle_scope().borrow_mut();

            // let context = scope.get_current_context();
            // let global = context.global(&mut scope);
            // global.get(scope, v8_string_key!(scope, "CloudstateTransaction"));
            // println!("polling");

            println!("committing");
            let _ = js_runtime.execute_script("<handle>", "globalThis.commit();");

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

    let cloudstate = js_runtime.op_state().borrow_mut().take::<ReDBCloudstate>();

    return Ok((cloudstate, result));
}
