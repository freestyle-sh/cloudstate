use deno_core::anyhow::Error;
use deno_core::*;
use redb::{Database, ReadableTable, TableDefinition, WriteTransaction};
use redis::Commands;
use std::collections::HashMap;

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
fn op_cloudstate_map_set(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
    #[string] value: String,
) -> Result<(), Error> {
    let connection = state
        .try_borrow_mut::<redis::Connection>()
        .expect("Redis connection should be in OpState.");

    let key = format!("maps:{}:{}", namespace, id).to_string();
    connection.hset(key, field, value)?;
    Ok(())
}

#[op2]
#[string]
fn op_cloudstate_map_get(
    state: &mut OpState,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
) -> Result<Option<String>, Error> {
    let connection = state
        .try_borrow_mut::<redis::Connection>()
        .expect("Redis connection should be in OpState.");

    let key: &String = &format!("maps:{}:{}", namespace, id).to_string();
    let result = connection.hget(key, field)?;

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
    let key = format!("{}:{}", namespace, alias).to_string();
    println!("op_cloudstate_object_root_get: {}", key);
    let result = table.get(key.as_str()).unwrap();
    let result = result.map(|s| s.value().to_string());
    println!("op_cloudstate_object_root_get: {:?}", result);
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
    let key = format!("{}:{}", namespace, alias).to_string();
    println!("op_cloudstate_object_root_set: {}", key);

    let write_txn = state
        .try_borrow_mut::<ReDBCloudstate>()
        .unwrap()
        .transactions
        .get_mut(&transaction_id)
        .unwrap();

    let mut table = write_txn.open_table(ROOTS_TABLE).unwrap();

    let _ = table.insert(key.as_str(), id.as_str()).unwrap();
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

struct ReDBCloudstate {
    pub db: Database,
    pub transactions: HashMap<String, WriteTransaction>,
}

// const OBJECTS_TABLE: TableDefinition<&str, Test> = TableDefinition::new("objects");
const ROOTS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("roots");

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
  state = | state: &mut OpState| {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let connection = client.get_connection().unwrap();
    let cs = ReDBCloudstate {
        db: Database::builder()
            .create("test-db.redb")
            // .create_with_backend(InMemoryBackend::default())
            .unwrap(),
        transactions: HashMap::<String, WriteTransaction>::new()
    };
    state.put(cs);
    state.put(connection);
  },
);
