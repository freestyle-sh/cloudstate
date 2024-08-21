use crate::bincode::Bincode;
use chrono::{DateTime, Utc};
use deno_core::anyhow::Error;
use deno_core::error::JsError;
use deno_core::*;
use redb::ReadableTable;
use redb::{Database, TableDefinition, WriteTransaction};
use serde::{Deserialize, Serialize};
use serde_v8::BigInt;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use url::Url;

#[op2]
fn op_cloudstate_object_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[serde] value: CloudstateObjectData,
) -> Result<(), Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(OBJECTS_TABLE).unwrap();

    let key = CloudstateObjectKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
    };

    let _ = table
        .insert(&key, CloudstateObjectValue { data: value })
        .unwrap();

    Ok(())
}

#[op2]
#[serde]
fn op_cloudstate_object_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
) -> Result<Option<CloudstateObjectData>, Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(OBJECTS_TABLE).unwrap();

    let key = CloudstateObjectKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
    };

    let result = table.get(key).unwrap();
    let result = result.map(|s| s.value().data);

    Ok(result)
}

#[op2(fast)]
fn op_cloudstate_map_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
    #[string] value: String,
) -> Result<(), Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(MAPS_TABLE).unwrap();

    let key = CloudstateMapFieldKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
        field: field.to_string(),
    };

    let _ = table
        .insert(&key, CloudstateMapFieldValue { data: value })
        .unwrap();
    Ok(())
}

#[op2]
#[string]
fn op_cloudstate_map_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
) -> Result<Option<String>, Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(MAPS_TABLE).unwrap();

    let key = CloudstateMapFieldKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
        field: field.to_string(),
    };

    let result = table.get(key).unwrap();
    let result = result.map(|s| s.value().data);

    Ok(result)
}

#[op2]
#[string]
fn op_cloudstate_object_root_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] alias: String,
) -> Result<Option<String>, Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(ROOTS_TABLE).unwrap();
    let key = CloudstateRootKey {
        namespace: namespace.to_string(),
        alias: alias.to_string(),
    };
    let result = table.get(key).unwrap();
    let result = result.map(|s| s.value().id);
    Ok(result)
}

#[op2(fast)]
fn op_cloudstate_object_root_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] alias: String,
    #[string] id: String,
) -> Result<(), Error> {
    let write_txn = state
        .try_borrow_mut::<ReDBCloudstate>()
        .unwrap()
        .transactions
        .get_mut(&transaction_id)
        .unwrap();

    let mut table = write_txn.open_table(ROOTS_TABLE).unwrap();

    let key = CloudstateRootKey {
        namespace: namespace.to_string(),
        alias: alias.to_string(),
    };

    let _ = table.insert(&key, CloudstateRootValue { id: id }).unwrap();
    Ok(())
}

#[op2(fast)]
#[string]
fn op_create_transaction(state: &mut OpState, #[string] id: String) -> Result<(), Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
    let write_txn = cs.db.begin_write().unwrap();
    cs.transactions.insert(id, write_txn);
    Ok(())
}

#[op2(fast)]
fn op_commit_transaction(state: &mut OpState, #[string] id: String) -> Result<(), Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
    let write_txn = cs.transactions.remove(&id).unwrap();
    write_txn.commit().unwrap();
    Ok(())
}

#[op2]
fn op_cloudstate_get_test_object<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &mut OpState,
    value: v8::Local<'a, v8::Object>,
) -> v8::Local<'a, v8::Object> {

    let newObj = v8::Object::new(scope);
    let key = v8::String::new(scope, "test").unwrap();
    let value = v8::String::new(scope, "test").unwrap();

    newObj.set(scope, key.into(), value.into());

    return newObj;

    
    // println!("{:?}", state);
    // let isolate_ptr = *state.try_borrow_mut::<*mut v8::OwnedIsolate>().unwrap();
    // let owned_isolate: Box<v8::OwnedIsolate> = unsafe { Box::from_raw(isolate_ptr) };
    // let isolate: v8::OwnedIsolate = *owned_isolate;

    // isolate

    // V8TaskSpawner::

    // let scope: v8::HandleScope<'_, deno_core::v8::Context> = v8::HandleScope::new(&mut isolate);
    // let context = v8::Context::new(&mut scope);

    // let obj = v8::Object::new(&mut scope);
    // println!("{:?}", isolate);
    // v8::HandleScope::new(&mut *isolate);

    // println!("{:?}", value);

    // return value;
}

pub struct ReDBCloudstate {
    pub db: Database,
    pub transactions: HashMap<String, WriteTransaction>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CloudstateRootKey {
    pub namespace: String,
    pub alias: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateRootValue {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CloudstateObjectKey {
    pub namespace: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateObjectValue {
    pub data: CloudstateObjectData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateObjectData {
    pub fields: HashMap<String, CloudstatePrimitiveData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum CloudstatePrimitiveData {
    Number(f64),
    String(String),
    Boolean(bool),
    BigInt(serde_v8::BigInt),
    Undefined,
    Null,
    Date(DateTime<Utc>),
    RegExp(String),
    URL(Url),
    Error(JsError),
    ObjectReference(String),
    MapReference(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CloudstateMapFieldKey {
    pub namespace: String,
    pub id: String,
    field: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateMapFieldValue {
    pub data: String,
}

pub const ROOTS_TABLE: TableDefinition<Bincode<CloudstateRootKey>, Bincode<CloudstateRootValue>> =
    TableDefinition::new("roots");

pub const OBJECTS_TABLE: TableDefinition<
    Bincode<CloudstateObjectKey>,
    Bincode<CloudstateObjectValue>,
> = TableDefinition::new("objects");

pub const MAPS_TABLE: TableDefinition<
    Bincode<CloudstateMapFieldKey>,
    Bincode<CloudstateMapFieldValue>,
> = TableDefinition::new("maps");

deno_core::extension!(
  cloudstate,
  ops = [
    op_cloudstate_object_set,
    op_cloudstate_object_get,
    op_cloudstate_object_root_set,
    op_cloudstate_object_root_get,
    op_cloudstate_map_set,
    op_cloudstate_map_get,
    op_create_transaction,
    op_commit_transaction,

    op_cloudstate_get_test_object
  ],
  esm_entry_point = "ext:cloudstate/cloudstate.js",
  esm = [ dir "src/extensions", "cloudstate.js" ],
);
