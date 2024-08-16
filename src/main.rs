use anyhow::Context;
use deno_core::anyhow::Error;
use deno_core::*;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[op2]
fn op_cloudstate_object_set(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] id: String,
    #[buffer] value: JsBuffer,
) -> Result<(), Error> {
    let map = state.try_borrow_mut::<HashMap<String, Vec<u8>>>().unwrap();
    map.insert(format!("{}:{}", namespace, id).to_string(), value.to_vec());
    Ok(())
}

#[op2]
#[buffer]
fn op_cloudstate_object_get(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] id: String,
) -> Result<Option<Vec<u8>>, Error> {
    let map = state.try_borrow_mut::<HashMap<String, Vec<u8>>>().unwrap();
    let id: &String = &format!("{}:{}", namespace, id).to_string();

    if let Some(value) = map.get(id) {
        Ok(Some(value.clone()))
    } else if let Ok(value) = fs::read(id) {
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

extension!(
  cloudstate,
  ops = [
    op_cloudstate_object_set,
    op_cloudstate_object_get,
  ],
  esm_entry_point = "ext:cloudstate/cloudstate.js",
  esm = [ dir "src", "cloudstate.js" ],
  state = | state: &mut OpState| {
    state.put(HashMap::<String, Vec::<u8>>::new());
  },
);

fn main() -> Result<(), Error> {
    let module_name = "test.js";
    let module_code = "
    const cloudstate = new Cloudstate('test');
    const object = { name: 'hello world' };
    cloudstate.setObject(object);
    cloudstate.setRoot(object, 'test');
    cloudstate.getRoot('test');
  "
    .to_string();

    let mut js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![cloudstate::init_ops_and_esm()],
        ..Default::default()
    });

    let main_module = resolve_path(
        module_name,
        &std::env::current_dir().context("Unable to get current working directory")?,
    )?;

    let future = async move {
        let mod_id = js_runtime
            .load_main_es_module_from_code(&main_module, module_code)
            .await?;

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
