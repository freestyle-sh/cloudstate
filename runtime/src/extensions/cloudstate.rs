use crate::backup::{backup_all_tables, BackupProgress};
use crate::blob_storage::{
  CloudstateBlobMetadata, CloudstateBlobStorage, CloudstateBlobValue,
};
use crate::tables::{ARRAYS_TABLE, MAPS_TABLE, OBJECTS_TABLE, ROOTS_TABLE};
use crate::v8_string_key;
use anyhow::anyhow;
use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use deno_core::anyhow::Error;
// use deno_core::error::JsError;

use deno_core::*;
use deno_error::JsErrorBox;
use redb::{
  AccessGuard, Database, Key, Range, ReadOnlyTable, ReadTransaction,
  ReadableTable, TableDefinition, Value, WriteTransaction,
};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::i32;
use std::ops::RangeBounds;
use std::path::Path;
use std::rc::Rc;
use std::result::Result::Ok;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};
use tracing::{debug, error, event, info, info_span, instrument};
use url::Url;
use v8::GetPropertyNamesArgs;

pub use js_spans::JavaScriptSpans;

mod js_spans;

pub struct TransactionContext {
  database: ReDBCloudstate,
  blob_storage: CloudstateBlobStorage,
  current_transaction: Option<Transaction>,
  read_only: bool,
}

impl TransactionContext {
  pub fn blob_storage(&self) -> &CloudstateBlobStorage {
    &self.blob_storage
  }
}

pub enum Transaction {
  Read(ReadTransaction),
  Write(WriteTransaction),
}

pub enum CloudstateTable<'a, K: Key + 'static, V: Value + 'static> {
  Read(ReadOnlyTable<K, V>),
  Write(redb::Table<'a, K, V>),
}

impl<'a, K, V> CloudstateTable<'a, K, V>
where
  K: Key + 'static,
  V: Value + 'static,
{
  pub fn insert(
    &mut self,
    key: impl std::borrow::Borrow<K::SelfType<'a>>,
    value: impl std::borrow::Borrow<V::SelfType<'a>>,
  ) -> Result<(), Error> {
    match self {
      CloudstateTable::Read(ref _table) => Ok(()), //panic!("Cannot insert into read-only table"),
      CloudstateTable::Write(ref mut table) => match table.insert(key, value) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
      },
    }
  }

  pub fn iter(&self) -> anyhow::Result<redb::Range<K, V>> {
    match self {
      CloudstateTable::Read(table) => table.iter().map_err(|e| e.into()),
      CloudstateTable::Write(table) => table.iter().map_err(|e| e.into()),
    }
  }

  pub fn remove<'b>(
    &mut self,
    key: impl std::borrow::Borrow<K::SelfType<'b>>,
  ) -> redb::Result<Option<AccessGuard<V>>>
  where
    K: 'b,
  {
    match self {
      CloudstateTable::Read(_table) => {
        panic!("Cannot remove during read-only transaction")
      }
      CloudstateTable::Write(table) => table.remove(key),
    }
  }

  pub fn get(
    &self,
    key: impl std::borrow::Borrow<K::SelfType<'a>>,
  ) -> anyhow::Result<Option<AccessGuard<V>>> {
    match self {
      CloudstateTable::Read(table) => table.get(key).map_err(|e| e.into()),
      CloudstateTable::Write(table) => table.get(key).map_err(|e| e.into()),
    }
  }
}

impl<'txn, K: Key + 'static, V: Value + 'static> CloudstateTable<'txn, K, V> {
  fn range<'a, KR>(
    &self,
    range: impl RangeBounds<KR> + 'a,
  ) -> redb::Result<Range<K, V>>
  where
    KR: Borrow<K::SelfType<'a>> + 'a,
  {
    match self {
      CloudstateTable::Read(table) => table.range(range),
      CloudstateTable::Write(table) => table.range(range),
    }
  }

  // fn first(&self) -> redb::Result<Option<(AccessGuard<K>, AccessGuard<V>)>> {
  //     match self {
  //         CloudstateTable::Read(table) => table.first(),
  //         CloudstateTable::Write(table) => table.first(),
  //     }
  // }

  // fn last(&self) -> redb::Result<Option<(AccessGuard<K>, AccessGuard<V>)>> {
  //     match self {
  //         CloudstateTable::Read(table) => table.last(),
  //         CloudstateTable::Write(table) => table.last(),
  //     }
  // }
}

impl Transaction {
  pub fn commit(self) -> Result<(), Error> {
    match self {
      Transaction::Read(transaction) => {
        transaction.close().map_err(|e| e.into())
      }
      Transaction::Write(transaction) => {
        transaction.commit().map_err(|e| e.into())
      }
    }
  }

  pub fn open_table<K: Key + 'static, V: Value + 'static>(
    &self,
    def: TableDefinition<K, V>,
  ) -> Result<CloudstateTable<K, V>, Error> {
    match self {
      Transaction::Read(transaction) => {
        let table = transaction.open_table(def)?;
        Ok(CloudstateTable::Read(table))
      }
      Transaction::Write(transaction) => {
        let table = transaction.open_table(def)?;
        Ok(CloudstateTable::Write(table))
      }
    }
  }
}

impl TransactionContext {
  pub fn new(database: ReDBCloudstate, storage: CloudstateBlobStorage) -> Self {
    Self {
      current_transaction: None,
      blob_storage: storage,
      database: database.clone(),
      read_only: false,
    }
  }

  pub fn set_read_only(&mut self) {
    if self.current_transaction.is_none() {
      self.read_only = true;
    }
  }

  #[instrument(skip(self))]
  pub fn get_or_create_transaction_mut(&mut self) -> &Transaction {
    // debug!("Checking for existing transaction");
    if self.current_transaction.is_none() {
      debug!("Creating new transaction");
      let db = self.database.get_database_mut();

      if self.read_only {
        let read_txn = db.begin_read().unwrap();
        self.current_transaction = Some(Transaction::Read(read_txn));
      } else {
        let write_txn = db.begin_write().unwrap();
        self.current_transaction = Some(Transaction::Write(write_txn));
      }
      self.current_transaction.as_mut().unwrap()
    } else {
      // debug!("Using existing transaction");
      self.current_transaction.as_mut().unwrap()
    }
  }

  #[instrument(skip(self))]
  pub fn commit_transaction(&mut self) {
    // debug!("Checking for transaction to commit");
    if let Some(transaction) = self.current_transaction.take() {
      debug!("Committing transaction");
      transaction.commit().unwrap();
    } else {
      debug!("No transaction to commit");
    }
  }
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_set_read_only(state: &mut OpState) {
  let cs = state.borrow_mut::<TransactionContext>();
  cs.set_read_only();
}

#[instrument(skip(state))]
#[op2]
fn op_cloudstate_object_set(
  state: &mut OpState,
  #[string] id: String,
  #[from_v8] value: CloudstateObjectData,
) -> Result<(), JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(OBJECTS_TABLE).unwrap();
  let key = CloudstateObjectKey { id };

  table
    .insert(&key, CloudstateObjectValue { data: value })
    .unwrap();

  Ok(())
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_object_get(
  state: &mut OpState,
  #[string] id: String,
) -> Result<CloudstateObjectData, JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = info_span!("open_table")
    .in_scope(|| transaction.open_table(OBJECTS_TABLE).unwrap());

  let key = CloudstateObjectKey { id };

  let result = info_span!("get").in_scope(|| table.get(key).unwrap());
  let result = info_span!("map").in_scope(|| result.map(|s| s.value().data));

  match result {
    Some(result) => Ok(result),
    None => Err(
      //anyhow!("Object not found")
      JsErrorBox::generic("Object not found"),
    ),
  }
}

#[instrument(skip(state))]
#[op2]
fn op_cloudstate_object_set_property(
  state: &mut OpState,
  #[string] id: String,
  #[string] property: String,
  #[from_v8] value: CloudstatePrimitiveData,
) -> Result<(), JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(OBJECTS_TABLE).unwrap();
  let key = CloudstateObjectKey { id };

  let mut object = table
    .get(key.clone())
    .map_err(|e| JsErrorBox::generic(e.to_string()))?
    .ok_or(anyhow!("Object not found"))
    .map_err(|e| JsErrorBox::generic(e.to_string()))?
    .value();

  object.data.fields.insert(property, value);

  table
    .insert(key, object.clone())
    .map_err(|e| JsErrorBox::generic(e.to_string()))?;

  Ok(())
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_array_reverse(
  state: &mut OpState,
  #[string] array_id: String,
) {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(ARRAYS_TABLE).unwrap();
  let keys: Vec<CloudstateArrayItemKey> = table
    .iter()
    .unwrap()
    .map(|entry| entry.unwrap().0.value())
    .filter(|key| key.id == array_id)
    .collect();

  let mut values = vec![];
  for key in &keys {
    let value = table.get(key).unwrap().unwrap().value().data;
    values.push(value);
  }

  for (_i, key) in keys.iter().enumerate() {
    let value = values.pop().unwrap();
    table
      .insert(
        &CloudstateArrayItemKey {
          id: key.id.clone(),
          index: key.index,
        },
        CloudstateArrayItemValue { data: value },
      )
      .unwrap();
  }
}

#[instrument(skip(state))]
#[op2]
#[serde]
fn op_cloudstate_list_roots(
  state: &mut OpState,
) -> Result<Vec<String>, JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(ROOTS_TABLE).unwrap();
  let mut roots: Vec<String> = Vec::new();

  for entry in table.iter().unwrap() {
    let (key, _value) = entry.unwrap();
    roots.push(key.value().alias);
  }

  Ok(roots)
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_array_pop(
  state: &mut OpState,
  #[string] array_id: String,
) -> Result<CloudstatePrimitiveData, JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(ARRAYS_TABLE).unwrap();
  let keys: Vec<CloudstateArrayItemKey> = table
    .iter()
    .unwrap()
    .map(|entry| entry.unwrap().0.value())
    .filter(|key| key.id == array_id)
    .collect();

  let length = keys.len() as i32;
  if length == 0 {
    return Ok(CloudstatePrimitiveData::Undefined);
  }

  if let Some(key) = keys.iter().find(|key| key.index == length - 1) {
    let value = table.remove(key).unwrap().unwrap().value().data;
    Ok(value)
  } else {
    Ok(CloudstatePrimitiveData::Undefined)
  }
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_array_shift(
  state: &mut OpState,
  #[string] array_id: String,
) -> CloudstatePrimitiveData {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(ARRAYS_TABLE).unwrap();
  let keys: Vec<CloudstateArrayItemKey> = table
    .iter()
    .unwrap()
    .map(|entry| entry.unwrap().0.value())
    .filter(|key| key.id == array_id)
    .collect();

  let mut return_value = None;
  for key in keys {
    let value = table.remove(&key).unwrap();
    let value = value.unwrap().value().data;
    if (key.index - 1) >= 0 {
      table
        .insert(
          &CloudstateArrayItemKey {
            id: key.id.clone(),
            index: key.index - 1,
          },
          CloudstateArrayItemValue { data: value },
        )
        .unwrap();
    } else {
      return_value = Some(value);
    }
  }

  return_value.unwrap_or(CloudstatePrimitiveData::Undefined)
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_cloudstate_get(
  state: &mut OpState,
  #[string] id: String,
) -> CloudstatePrimitiveData {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(OBJECTS_TABLE).unwrap();

  let result = table.iter().unwrap().find(|value| {
    value
      .as_ref()
      .unwrap()
      .1
      .value()
      .data
      .fields
      .get("id")
      .map_or(false, |id_value| {
        id_value == &CloudstatePrimitiveData::String(id.clone())
      })
  });

  match result {
    Some(result) => CloudstatePrimitiveData::ObjectReference(ObjectReference {
      id: result.unwrap().0.value().id.clone(),
    }),
    None => CloudstatePrimitiveData::Undefined,
  }
}

#[instrument(skip(state))]
#[op2]
fn op_cloudstate_map_set(
  state: &mut OpState,
  #[string] id: String,
  #[string] field: String,
  #[from_v8] value: CloudstatePrimitiveData,
) -> Result<(), JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(MAPS_TABLE).unwrap();
  let key = CloudstateMapFieldKey { id, field };

  table
    .insert(&key, CloudstateMapFieldValue { data: value })
    .unwrap();
  Ok(())
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_map_delete(
  state: &mut OpState,
  #[string] map_id: String,
  #[string] key: String,
) -> bool {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(MAPS_TABLE).unwrap();
  let key = CloudstateMapFieldKey {
    id: map_id,
    field: key,
  };

  let was_removed = table.remove(&key).unwrap_or(None).is_some();
  println!("{:?} was_removed: {}", key.field, was_removed);
  was_removed
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_map_clear(state: &mut OpState, #[string] map_id: String) {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(MAPS_TABLE).unwrap();
  let keys: Vec<CloudstateMapFieldKey> = table
    .iter()
    .unwrap()
    .map(|entry| entry.unwrap().0.value())
    .filter(|key| key.id == map_id)
    .collect();

  for key in keys {
    table.remove(&key).unwrap();
  }
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_map_get(
  state: &mut OpState,
  #[string] id: String,
  #[string] field: String,
) -> CloudstatePrimitiveData {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(MAPS_TABLE).unwrap();
  let key = CloudstateMapFieldKey { id, field };

  let primitive = match table.get(key).unwrap_or(None) {
    Some(value) => value.value().data,
    None => CloudstatePrimitiveData::Undefined,
  };

  primitive
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_map_has(
  state: &mut OpState,
  #[string] id: String,
  #[string] field: String,
) -> CloudstatePrimitiveData {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(MAPS_TABLE).unwrap();
  let key = CloudstateMapFieldKey { id, field };

  let primitive = match table.get(key).unwrap_or(None) {
    Some(_) => CloudstatePrimitiveData::Boolean(true),
    None => CloudstatePrimitiveData::Boolean(false),
  };

  primitive
}

#[instrument(skip(state))]
#[op2]
fn op_cloudstate_array_set(
  state: &mut OpState,
  #[string] id: String,
  index: i32,
  #[from_v8] value: CloudstatePrimitiveData,
) -> Result<(), JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(ARRAYS_TABLE).unwrap();
  let key = CloudstateArrayItemKey { id, index };

  table
    .insert(&key, CloudstateArrayItemValue { data: value })
    .unwrap();
  Ok(())
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_array_length(
  state: &mut OpState,
  #[string] id: String,
) -> i32 {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(ARRAYS_TABLE).unwrap();

  let count = table
    .range(
      CloudstateArrayItemKey {
        id: id.clone(),
        index: 0,
      }..CloudstateArrayItemKey {
        id: id.clone() + "\u{0}",
        index: 0,
      },
    )
    .unwrap()
    .count();

  count as i32
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_array_get(
  state: &mut OpState,
  #[string] id: String,
  index: i32,
) -> CloudstatePrimitiveData {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(ARRAYS_TABLE).unwrap();
  let key = CloudstateArrayItemKey { id, index };

  let result = table.get(key).unwrap();
  let result = result.map(|s| s.value().data);

  match result {
    Some(result) => result,
    None => CloudstatePrimitiveData::Undefined,
  }
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_map_size(
  state: &mut OpState,
  #[string] map_id: String,
) -> Result<i32, JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(MAPS_TABLE).unwrap();
  let count = table
    .iter()
    .unwrap()
    .map(|entry| entry.unwrap())
    .filter(|(key, _value)| key.value().id == map_id)
    .count();

  Ok(count as i32)
}

#[instrument(skip(state))]
#[op2]
#[string]
fn op_cloudstate_object_root_get(
  state: &mut OpState,
  #[string] alias: String,
) -> Result<Option<String>, JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(ROOTS_TABLE).unwrap();
  let key = CloudstateRootKey { alias };

  let result = table.get(key).unwrap();
  let result = result.map(|s| s.value().id);
  Ok(result)
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_object_root_set(
  state: &mut OpState,
  #[string] alias: String,
  #[string] id: String,
) -> Result<(), JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let mut table = transaction.open_table(ROOTS_TABLE).unwrap();
  let key = CloudstateRootKey { alias };

  table.insert(&key, CloudstateRootValue { id }).unwrap();
  Ok(())
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_commit_transaction(
  state: &mut OpState,
) -> Result<(), JsErrorBox> {
  event!(tracing::Level::DEBUG, "Committing transaction");
  let cs = state.borrow_mut::<TransactionContext>();
  cs.commit_transaction();
  debug!("Transaction committed");
  Ok(())
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_map_values(
  state: &mut OpState,
  #[string] map_id: String,
) -> Result<CloudstatePrimitiveDataVec, JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(MAPS_TABLE).unwrap();
  let mut values = vec![];

  for entry in table.iter().unwrap() {
    let (key, value) = entry.unwrap();
    event!(tracing::Level::DEBUG, "Key: {:?}", key.value());
    event!(tracing::Level::DEBUG, "Value: {:?}", value.value());

    if key.value().id == map_id {
      values.push(value.value().data);
    }
  }

  Ok(values.into())
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_map_keys(
  state: &mut OpState,
  #[string] map_id: String,
) -> Result<CloudstatePrimitiveDataVec, JsErrorBox> {
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(MAPS_TABLE).unwrap();
  let mut keys = vec![];

  for entry in table.iter().unwrap() {
    let (key, _value) = entry.unwrap();
    if key.value().id == map_id {
      //TODO: UPDATE WHEN MAP KEYS ARE MOVED TO ANY PRIMITIVE
      keys.push(CloudstatePrimitiveData::String(key.value().field));
    }
  }

  Ok(keys.into())
}

#[instrument(skip(state))]
#[op2]
#[to_v8]
fn op_cloudstate_map_entries(
  state: &mut OpState,
  #[string] map_id: String,
) -> Result<CloudstateEntriesVec, JsErrorBox> {
  event!(tracing::Level::DEBUG, "Getting map entries");
  let cs = state.borrow_mut::<TransactionContext>();
  let transaction = cs.get_or_create_transaction_mut();

  let table = transaction.open_table(MAPS_TABLE).unwrap();
  let mut entries: Vec<Vec<CloudstatePrimitiveData>> = vec![];

  for entry in table.iter().unwrap() {
    let (key, value) = entry.unwrap();
    if key.value().id == map_id {
      entries.push(vec![
        CloudstatePrimitiveData::String(key.value().field),
        value.value().data,
      ]);
    }
  }

  Ok(CloudstateEntriesVec::from(entries))
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_blob_set(
  state: Rc<RefCell<OpState>>,
  #[string] blob_id: String,
  #[string] blob_type: String,
  #[arraybuffer] blob_data: &[u8], // #[buffer] blob_data: JsBuffer,
) -> Result<(), deno_error::JsErrorBox> {
  let mut state = RefCell::borrow_mut(&state);

  let transaction_context = state.borrow_mut::<TransactionContext>();
  let storage = transaction_context.blob_storage().clone();
  let transaction = transaction_context.get_or_create_transaction_mut();

  let data = blob_data.to_vec();

  storage
    .put_blob(
      &blob_id,
      transaction,
      CloudstateBlobValue { data },
      CloudstateBlobMetadata { type_: blob_type },
    )
    .map_err(|e| JsErrorBox::generic(e.to_string()))?;

  Ok(())
}

#[instrument(skip(state))]
#[op2]
#[arraybuffer]
fn op_cloudstate_blob_slice(
  state: Rc<RefCell<OpState>>,
  #[string] blob_id: String,
  start: Option<i32>,
  end: Option<i32>,
) -> Result<Vec<u8>, JsErrorBox> {
  let mut state = RefCell::borrow_mut(&state);

  let transaction_context = state.borrow_mut::<TransactionContext>();
  let storage = transaction_context.blob_storage().clone();

  let result = storage
    .get_blob_slice(&blob_id, start, end)
    .map_err(|e| JsErrorBox::generic(format!("{:?}", e)))?;
  Ok(result)
}

#[instrument(skip(state))]
#[op2()]
#[arraybuffer]
fn op_cloudstate_blob_get_array_buffer(
  state: &mut OpState,
  #[string] blob_id: String,
) -> Result<Vec<u8>, JsErrorBox> {
  let blob_store = state.borrow_mut::<TransactionContext>().blob_storage();
  let result = blob_store
    .get_blob_data(&blob_id)
    .map_err(|e| JsErrorBox::generic(format!("{:?}", e)))?
    .data;

  Ok(result)
}

#[instrument(skip(state))]
#[op2()]
#[buffer]
fn op_cloudstate_blob_get_uint8array(
  state: &mut OpState,
  #[string] blob_id: String,
) -> Result<Vec<u8>, JsErrorBox> {
  let blob_store = state.borrow_mut::<TransactionContext>().blob_storage();
  let result = blob_store
    .get_blob_data(&blob_id)
    .map_err(|e| JsErrorBox::generic(format!("{:?}", e)))?
    .data;

  Ok(result)
}

#[instrument(skip(state))]
#[op2()]
#[string]
fn op_cloudstate_blob_get_text(
  state: &mut OpState,
  #[string] blob_id: String,
) -> Result<String, JsErrorBox> {
  let blob_store = state.borrow_mut::<TransactionContext>().blob_storage();
  let result = blob_store
    .get_blob_data(&blob_id)
    .map_err(|e| JsErrorBox::generic(format!("{:?}", e)))?
    .data;
  Ok(String::from_utf8(result).unwrap())
}

#[instrument(skip(state))]
#[op2(fast)]
fn op_cloudstate_blob_get_size(
  state: &mut OpState,
  #[string] blob_id: String,
) -> Result<i32, JsErrorBox> {
  let blob_store = state.borrow_mut::<TransactionContext>().blob_storage();
  let result = blob_store
    .get_blob_size(&blob_id)
    .map_err(|e| JsErrorBox::generic(format!("{:?}", e)))?;
  Ok(result as i32)
}

#[instrument(skip(state))]
#[op2]
#[string]
fn op_cloudstate_blob_get_type(
  state: Rc<RefCell<OpState>>,
  #[string] blob_id: String,
) -> Result<String, JsErrorBox> {
  let mut state = RefCell::borrow_mut(&state);

  let transaction_context = state.borrow_mut::<TransactionContext>();
  let storage = transaction_context.blob_storage().clone();
  let transaction = transaction_context.get_or_create_transaction_mut();

  match storage.get_blob_metadata(&blob_id, transaction) {
    Ok(metadata) => Ok(metadata.type_),
    Err(_) => Err(JsErrorBox::generic("Blob not found")),
  }
}

#[instrument(skip(_state))]
#[op2(fast)]
pub fn op_print_with_tracing(
  _state: &mut OpState,
  #[string] msg: &str,
  is_err: bool,
) {
  // tracing
  if is_err {
    error!("{}", msg);
  } else {
    info!("{}", msg);
  }
}

// #[instrument(skip(state))]
#[op2(fast)]
pub fn op_tracing_span_finish(state: &mut OpState) {
  let spans = state.borrow_mut::<JavaScriptSpans>();
  spans.pop_span();
}

#[derive(
  Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub struct CloudstateBlobKey {
  pub id: String,
}

impl From<&str> for CloudstateBlobKey {
  fn from(id: &str) -> Self {
    Self { id: id.to_string() }
  }
}

impl From<String> for CloudstateBlobKey {
  fn from(id: String) -> Self {
    Self { id }
  }
}

#[derive(Clone, Debug)]
pub struct ReDBCloudstate {
  db: Arc<Mutex<Database>>,
}

impl ReDBCloudstate {
  pub fn new(db: Arc<Mutex<Database>>) -> Self {
    Self { db }
  }

  pub fn get_database_mut(&self) -> MutexGuard<Database> {
    self.db.lock().unwrap()
  }

  pub fn backup<'a>(
    &self,
    path: impl AsRef<Path>,
    progress_callback: &mut Option<Box<dyn FnMut(BackupProgress) + 'a>>,
  ) -> Result<(), Error> {
    let db = self.get_database_mut();
    let backup_db = Database::create(path)?;
    let read = db.begin_read().unwrap();
    let write = backup_db.begin_write().unwrap();

    backup_all_tables(&read, &write, progress_callback)?;

    read.close().unwrap();
    write.commit().unwrap();

    Ok(())

    //db.begin_read();
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CloudstateRootKey {
  pub alias: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateRootValue {
  pub id: String,
}

#[derive(
  Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub struct CloudstateObjectKey {
  pub id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CloudstateObjectValue {
  pub data: CloudstateObjectData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CloudstateObjectData {
  pub fields: HashMap<String, CloudstatePrimitiveData>,
  pub constructor_name: Option<String>,
}

impl ToV8<'_> for CloudstateObjectData {
  fn to_v8<'a>(
    self,
    scope: &mut v8::HandleScope<'a>,
  ) -> Result<deno_core::v8::Local<'a, deno_core::v8::Value>, JsErrorBox> {
    let object = v8::Object::new(scope);
    for (key, value) in self.fields.iter() {
      let key =
        v8::Local::<v8::Value>::from(v8::String::new(scope, key).unwrap());
      let value = value.clone().to_v8(scope).unwrap();
      object.set(scope, key, value);
    }

    if let Some(constructor_name) = &self.constructor_name {
      // TODO: we shouldn't be passing data through abnormal channels like this
      let constructor_name_key =
        v8::String::new(scope, "__cloudstate__constructorName").unwrap();
      let constructor_name = v8::String::new(scope, constructor_name).unwrap();
      object.set(scope, constructor_name_key.into(), constructor_name.into());
    }

    Ok(v8::Local::<v8::Value>::from(object))
  }

  type Error = JsErrorBox;
}

impl FromV8<'_> for CloudstateObjectData {
  fn from_v8<'a>(
    scope: &mut v8::HandleScope<'a>,
    value: v8::Local<'a, v8::Value>,
  ) -> Result<Self, Self::Error> {
    let object = v8::Local::<v8::Object>::try_from(value).unwrap();
    let mut fields = HashMap::new();
    let array = object
      .get_own_property_names(
        scope,
        GetPropertyNamesArgs {
          ..Default::default()
        },
      )
      .unwrap();

    let length = array.length();

    for i in 0..length {
      let key = array.get_index(scope, i).unwrap();
      let value = object.get(scope, key).unwrap();
      let value = CloudstatePrimitiveData::from_v8(scope, value).unwrap();
      fields.insert(key.to_rust_string_lossy(scope), value);
    }

    let constructor_key = v8_string_key!(scope, "constructor");
    let name_key = v8_string_key!(scope, "name");
    Ok(CloudstateObjectData {
      fields,
      constructor_name: match v8::Local::<v8::Object>::try_from(
        object
          // .get_constructor_name()
          .get(scope, constructor_key)
          .unwrap(),
      )
      .unwrap()
      .get(scope, name_key)
      .unwrap()
      .to_rust_string_lossy(scope)
      .as_str()
      {
        "Object" => None,
        constructor_name => Some(constructor_name.to_string()),
      },
    })
  }

  type Error = JsErrorBox;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CloudstatePrimitiveData {
  Number(f64),
  String(String),
  Boolean(bool),
  BigInt(Box<[u64]>),
  Undefined,
  Null,
  Date(DateTime<Utc>),
  Blob(Blob),
  Url(Url),
  // Error(JsError),
  ObjectReference(ObjectReference),
  MapReference(String),
  ArrayReference(String),
}

#[derive(
  Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub struct ObjectReference {
  pub id: String,
}

impl ObjectReference {
  pub fn _hydrate(&self) {}
}

#[derive(
  Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone,
)]
pub struct Blob {
  pub id: String,
}

struct CloudstatePrimitiveDataVec {
  data: Vec<CloudstatePrimitiveData>,
}

impl From<Vec<CloudstatePrimitiveData>> for CloudstatePrimitiveDataVec {
  fn from(data: Vec<CloudstatePrimitiveData>) -> Self {
    CloudstatePrimitiveDataVec { data }
  }
}

struct CloudstateEntriesVec {
  data: Vec<Vec<CloudstatePrimitiveData>>,
}

impl From<Vec<Vec<CloudstatePrimitiveData>>> for CloudstateEntriesVec {
  fn from(data: Vec<Vec<CloudstatePrimitiveData>>) -> Self {
    CloudstateEntriesVec { data }
  }
}

impl ToV8<'_> for CloudstateEntriesVec {
  fn to_v8<'a>(
    self,
    scope: &mut v8::HandleScope<'a>,
  ) -> Result<v8::Local<'a, v8::Value>, JsErrorBox> {
    let array = v8::Array::new(scope, self.data.len() as i32);
    for (i, entry) in self.data.iter().enumerate() {
      let key = entry[0].clone();
      let value = entry[1].clone();

      let key = key.clone().to_v8(scope).unwrap();
      let value = value.clone().to_v8(scope).unwrap();

      let entry = v8::Array::new(scope, 2);
      entry.set_index(scope, 0, key);
      entry.set_index(scope, 1, value);

      array.set_index(scope, i as u32, entry.into());
    }
    Ok(array.into())
  }

  type Error = JsErrorBox;
}

impl ToV8<'_> for CloudstatePrimitiveDataVec {
  type Error = JsErrorBox;

  fn to_v8<'a>(
    self,
    scope: &mut v8::HandleScope<'a>,
  ) -> Result<v8::Local<'a, v8::Value>, Self::Error> {
    let array = v8::Array::new(scope, self.data.len() as i32);
    for (i, value) in self.data.iter().enumerate() {
      let val = value.clone().to_v8(scope).unwrap();
      array.set_index(scope, i as u32, val);
    }
    Ok(array.into())
  }
}

impl ToV8<'_> for CloudstatePrimitiveData {
  fn to_v8<'a>(
    self,
    scope: &mut v8::HandleScope<'a>,
  ) -> Result<v8::Local<'a, v8::Value>, JsErrorBox> {
    Ok(match self {
      CloudstatePrimitiveData::Date(value) => {
        deno_core::v8::Local::<v8::Value>::from(
          v8::Date::new(scope, value.timestamp_millis() as f64).unwrap(),
        )
      }
      CloudstatePrimitiveData::Number(value) => {
        v8::Number::new(scope, value).into()
      }
      CloudstatePrimitiveData::String(value) => {
        v8::String::new(scope, &value).unwrap().into()
      }
      CloudstatePrimitiveData::Boolean(value) => {
        v8::Boolean::new(scope, value).into()
      }
      CloudstatePrimitiveData::BigInt(value) => {
        v8::BigInt::new_from_words(scope, false, &value)
          .unwrap()
          .into()
      }
      CloudstatePrimitiveData::Undefined => v8::undefined(scope).into(),
      CloudstatePrimitiveData::Null => v8::null(scope).into(),
      CloudstatePrimitiveData::Url(value) => {
        v8::String::new(scope, value.as_str()).unwrap().into()
      }
      CloudstatePrimitiveData::Blob(value) => {
        let context = scope.get_current_context();
        let export_name = "CloudstateBlobReference";

        let class = {
          let global = context.global(scope);

          let export_name = v8::String::new(scope, export_name).unwrap().into();
          global
            .get(scope, export_name)
            .expect("CloudstateBlobReference class should be globally defined")
        };

        let prototype_key = v8::String::new(scope, "prototype").unwrap().into();
        let prototype = v8::Local::<v8::Function>::try_from(class)
          .unwrap()
          .get(scope, prototype_key)
          .unwrap();

        let object = v8::Object::new(scope);
        object.set_prototype(scope, prototype).unwrap();
        let key = v8::String::new(scope, "blobId").unwrap().into();
        let value = v8::String::new(scope, &value.id).unwrap().into();
        object.set(scope, key, value);
        object.into()
      }
      // CloudstatePrimitiveData::Error(value) => v8::Error::new(scope, value.clone()).into(),
      CloudstatePrimitiveData::ObjectReference(value) => {
        let context = scope.get_current_context();
        let export_name = "CloudstateObjectReference";

        let class = {
          let global = context.global(scope);

          let export_name = v8::String::new(scope, export_name).unwrap().into();
          global.get(scope, export_name).expect(
            "CloudstateObjectReference class should be globally defined",
          )
        };

        let prototype_key = v8::String::new(scope, "prototype").unwrap().into();
        let prototype = v8::Local::<v8::Function>::try_from(class)
          .unwrap()
          .get(scope, prototype_key)
          .unwrap();

        let object = v8::Object::new(scope);
        object.set_prototype(scope, prototype).unwrap();
        let key = v8::String::new(scope, "objectId").unwrap().into();
        let object_value = v8::String::new(scope, &value.id).unwrap().into();

        object.set(scope, key, object_value);

        object.into()
      }
      CloudstatePrimitiveData::ArrayReference(value) => {
        let context = scope.get_current_context();
        let export_name = "CloudstateArrayReference";

        let class = {
          let global = context.global(scope);

          let export_name = v8::String::new(scope, export_name).unwrap().into();
          global
            .get(scope, export_name)
            .expect("CloudstateArrayReference class should be globally defined")
        };

        let prototype_key = v8::String::new(scope, "prototype").unwrap().into();
        let prototype = v8::Local::<v8::Function>::try_from(class)
          .unwrap()
          .get(scope, prototype_key)
          .unwrap();

        let object = v8::Object::new(scope);
        object.set_prototype(scope, prototype).unwrap();
        let key = v8::String::new(scope, "objectId").unwrap().into();
        let value = v8::String::new(scope, &value).unwrap().into();
        object.set(scope, key, value);
        object.into()
      }
      CloudstatePrimitiveData::MapReference(value) => {
        let context = scope.get_current_context();
        let export_name = "CloudstateMapReference";

        let class = {
          let global = context.global(scope);

          let export_name = v8::String::new(scope, export_name).unwrap().into();
          global
            .get(scope, export_name)
            .expect("CloudstateMapReference class should be globally defined")
        };

        let prototype_key = v8::String::new(scope, "prototype").unwrap().into();
        let prototype = v8::Local::<v8::Function>::try_from(class)
          .unwrap()
          .get(scope, prototype_key)
          .unwrap();

        let object = v8::Object::new(scope);
        object.set_prototype(scope, prototype).unwrap();
        let key = v8::String::new(scope, "objectId").unwrap().into();
        let value = v8::String::new(scope, &value).unwrap().into();
        object.set(scope, key, value);
        object.into()
      }
    })
  }

  type Error = JsErrorBox;
}

impl FromV8<'_> for CloudstatePrimitiveData {
  fn from_v8<'a>(
    scope: &mut v8::HandleScope<'a>,
    value: v8::Local<'a, v8::Value>,
  ) -> Result<Self, Self::Error> {
    if value.is_null() {
      Ok(CloudstatePrimitiveData::Null)
    } else if value.is_undefined() {
      Ok(CloudstatePrimitiveData::Undefined)
    } else if value.is_big_int() {
      let bigint = v8::Local::<v8::BigInt>::try_from(value).unwrap();
      let length = bigint.word_count();
      let mut value = vec![0; length];
      bigint.to_words_array(&mut value);
      let value = value.into_boxed_slice();
      Ok(CloudstatePrimitiveData::BigInt(value))
    } else if value.is_number() {
      let number = v8::Local::<v8::Number>::try_from(value).unwrap();
      let value = number.value();
      Ok(CloudstatePrimitiveData::Number(value))
    } else if value.is_string() {
      let string = v8::Local::<v8::String>::try_from(value).unwrap();
      let value = string.to_rust_string_lossy(scope);
      Ok(CloudstatePrimitiveData::String(value))
    } else if value.is_boolean() {
      let boolean = v8::Local::<v8::Boolean>::try_from(value).unwrap();
      let value = boolean.boolean_value(scope);
      Ok(CloudstatePrimitiveData::Boolean(value))
    } else if value.is_date() {
      let date = v8::Local::<v8::Date>::try_from(value).unwrap();
      let value = date.value_of();
      Ok(CloudstatePrimitiveData::Date(
        Utc.timestamp_millis_opt(value as i64).unwrap(),
      ))
    } else if value.is_undefined() {
      Ok(CloudstatePrimitiveData::Undefined)
    } else if value.is_object() {
      let object = v8::Local::<v8::Object>::try_from(value).unwrap();
      let constructor =
        object.get_constructor_name().to_rust_string_lossy(scope);
      match constructor.as_str() {
        "CloudstateMapReference" => {
          let key = v8::String::new(scope, "objectId").unwrap().into();
          return Ok(CloudstatePrimitiveData::MapReference(
            v8::Local::<v8::String>::try_from(object.get(scope, key).unwrap())
              .unwrap()
              .to_rust_string_lossy(scope),
          ));
        }
        "CloudstateBlobReference" => {
          let key = v8::String::new(scope, "blobId").unwrap().into();
          return Ok(CloudstatePrimitiveData::Blob(Blob {
            id: v8::Local::<v8::String>::try_from(
              object.get(scope, key).unwrap(),
            )
            .unwrap()
            .to_rust_string_lossy(scope),
          }));
        }
        "CloudstateObjectReference" => {
          let object_key = v8::String::new(scope, "objectId").unwrap().into();
          let constructor_name_key =
            v8::String::new(scope, "constructorName").unwrap();

          let constructor_name =
            object.get(scope, constructor_name_key.into()).unwrap();
          if constructor_name.is_undefined() {
            None
          } else {
            Some(
              v8::Local::<v8::String>::try_from(constructor_name)
                .unwrap()
                .to_rust_string_lossy(scope),
            )
          };

          let object_reference =
            CloudstatePrimitiveData::ObjectReference(ObjectReference {
              id: v8::Local::<v8::String>::try_from(
                object.get(scope, object_key).unwrap(),
              )
              .unwrap()
              .to_rust_string_lossy(scope),
            });

          return Ok(object_reference);
        }
        "CloudstateArrayReference" => {
          let key = v8::String::new(scope, "objectId").unwrap().into();
          return Ok(CloudstatePrimitiveData::ArrayReference(
            v8::Local::<v8::String>::try_from(object.get(scope, key).unwrap())
              .unwrap()
              .to_rust_string_lossy(scope),
          ));
        }
        _ => panic!("Custom classes not implemented yet"),
      }
    } else {
      let msg_str = v8::String::new(scope, "Not implemented").unwrap();
      let msg = v8::Exception::error(scope, msg_str);
      Err(JsErrorBox::generic(
        format!(
          "Conversion from V8 to CloudstatePrimitiveData not implemented for {:?}",
          msg
        )
      ))
    }
  }

  type Error = JsErrorBox;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CloudstateMapFieldKey {
  pub id: String,
  pub field: String,
}

impl PartialOrd for CloudstateMapFieldKey {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for CloudstateMapFieldKey {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .id
      .cmp(&other.id)
      .then_with(|| self.field.cmp(&other.field))
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateMapFieldValue {
  pub data: CloudstatePrimitiveData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CloudstateArrayItemKey {
  pub id: String,
  pub index: i32,
}

impl PartialOrd for CloudstateArrayItemKey {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for CloudstateArrayItemKey {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .id
      .cmp(&other.id)
      .then_with(|| self.index.cmp(&other.index))
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateArrayItemValue {
  pub data: CloudstatePrimitiveData,
}

deno_core::extension!(
  cloudstate,
  ops = [
    op_cloudstate_array_get,
    op_cloudstate_array_length,
    op_cloudstate_array_pop,
    op_cloudstate_array_reverse,
    op_cloudstate_array_set,
    op_cloudstate_array_shift,
    op_cloudstate_cloudstate_get,
    op_cloudstate_commit_transaction,
    op_cloudstate_map_clear,
    op_cloudstate_map_delete,
    op_cloudstate_map_entries,
    op_cloudstate_map_get,
    op_cloudstate_map_has,
    op_cloudstate_map_keys,
    op_cloudstate_map_set,
    op_cloudstate_map_size,
    op_cloudstate_map_values,
    op_cloudstate_object_get,
    op_cloudstate_object_root_get,
    op_cloudstate_object_root_set,
    op_cloudstate_object_set,
    op_cloudstate_object_set_property,
    op_cloudstate_blob_get_array_buffer,
    op_cloudstate_blob_get_uint8array,
    op_cloudstate_blob_get_text,
    op_cloudstate_blob_set,
    op_cloudstate_blob_slice,
    op_cloudstate_blob_get_size,
    op_cloudstate_blob_get_type,
    op_cloudstate_list_roots,
    op_cloudstate_set_read_only,

    op_tracing_span_finish,

    js_spans::op_tracing_span_hydrate,
    js_spans::op_tracing_span_get_map,
    js_spans::op_tracing_span_get_object,
    js_spans::op_tracing_span_pack_to_reference_or_primitive,
    js_spans::op_tracing_span_unpack_from_reference,
    js_spans::op_tracing_span_get_cloudstate,
    js_spans::op_tracing_span_set_object,
    js_spans::op_tracing_span_get_array,
    js_spans::op_tracing_span_export_object,
    js_spans::op_tracing_span_set_root,
    js_spans::op_tracing_span_get_root,
    js_spans::op_tracing_span_array_filter,
    js_spans::op_tracing_span_array_splice,
    js_spans::op_tracing_span_commit
  ],
  esm_entry_point = "ext:cloudstate/cloudstate.js",
  esm = [ dir "src/extensions", "cloudstate.js" ],
  middleware = |op| match op.name {
    "op_print" => op_print_with_tracing(),
    _ => op,
  },

);
