use deno_core::anyhow::Error;
use deno_core::*;
use redis::Commands;
use std::path::Path;
use std::rc::Rc;

#[op2(fast)]
fn op_cloudstate_object_set(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] id: String,
    #[string] value: String,
) -> Result<(), Error> {
    let connection: &mut redis::Connection = state
        .try_borrow_mut::<redis::Connection>()
        .expect("Redis connection should be in OpState.");

    let key = format!("objects:{}:{}", namespace, id).to_string();
    connection.set(key, value)?;

    Ok(())
}

#[op2]
#[string]
fn op_cloudstate_object_get(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] id: String,
) -> Result<Option<String>, Error> {
    let connection = state
        .try_borrow_mut::<redis::Connection>()
        .expect("Redis connection should be in OpState.");
    let key: &String = &format!("objects:{}:{}", namespace, id).to_string();

    let result = connection.get::<String, Option<String>>(key.to_string())?;

    Ok(result)
}

#[op2(fast)]
fn op_cloudstate_object_root_set(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] alias: String,
    #[string] id: String,
) -> Result<(), Error> {
    let connection = state
        .try_borrow_mut::<redis::Connection>()
        .expect("Redis connection should be in OpState.");

    let key = format!("roots:{}:{}", namespace, alias).to_string();
    connection.set(key, id)?;

    Ok(())
}

#[op2]
#[string]
fn op_cloudstate_object_root_get(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] alias: String,
) -> Result<Option<String>, Error> {
    let connection = state
        .try_borrow_mut::<redis::Connection>()
        .expect("Redis connection should be in OpState.");

    let key: &String = &format!("roots:{}:{}", namespace, alias).to_string();
    let result = connection.get::<String, Option<String>>(key.to_string())?;

    Ok(result)
}

deno_core::extension!(
  cloudstate,
  ops = [
    op_cloudstate_object_set,
    op_cloudstate_object_get,
    op_cloudstate_object_root_set,
    op_cloudstate_object_root_get,
  ],
  esm_entry_point = "ext:cloudstate/cloudstate.js",
  esm = [ dir "src/extensions", "cloudstate.js" ],
  state = | state: &mut OpState| {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Redis should be running.");
    let connection = client.get_connection().expect("Redis connection should be available.");
    state.put(connection);
  },
);
