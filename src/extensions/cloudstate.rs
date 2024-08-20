use crate::bincode::Bincode;
use deno_core::anyhow::Error;
use deno_core::*;
use redb::ReadableTable;
use redb::{Database, TableDefinition, WriteTransaction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[op2(fast)]
fn op_cloudstate_object_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] value: String,
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
#[string]
fn op_cloudstate_object_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
) -> Result<Option<String>, Error> {
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
    pub data: String,
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
  ],
  esm_entry_point = "ext:cloudstate/cloudstate.js",
  esm = [ dir "src/extensions", "cloudstate.js" ],
);
