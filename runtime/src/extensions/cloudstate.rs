use crate::tables::{ARRAYS_TABLE, MAPS_TABLE, OBJECTS_TABLE, ROOTS_TABLE};
use chrono::{DateTime, TimeZone, Utc};
use deno_core::anyhow::Error;
use deno_core::error::JsError;
use deno_core::*;
use redb::ReadableTable;
use redb::{Database, WriteTransaction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::i32;
use std::sync::Arc;
use std::sync::Mutex;
use url::Url;
use v8::{Function, GetPropertyNamesArgs, HandleScope};

#[op2]
fn op_cloudstate_object_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[from_v8] value: CloudstateObjectData,
) -> Result<(), Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

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
#[to_v8]
fn op_cloudstate_object_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
) -> Result<CloudstateObjectData, Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(OBJECTS_TABLE).unwrap();

    let key = CloudstateObjectKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
    };

    let result = table.get(key).unwrap();
    let result = result.map(|s| s.value().data);

    Ok(result.unwrap())
}

#[op2(fast)]
fn op_cloudstate_array_reverse(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] array_id: String,
) {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();

    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(ARRAYS_TABLE).unwrap();

    let keys: Vec<CloudstateArrayItemKey> = table
        .iter()
        .unwrap()
        .map(|entry| entry.unwrap().0.value())
        .filter(|key| key.id == array_id && key.namespace == namespace)
        .collect();

    let mut values = vec![];

    for key in &keys {
        let value = table.get(key).unwrap().unwrap().value().data;
        values.push(value);
    }

    for (_i, key) in keys.iter().enumerate() {
        let value = values.pop().unwrap(); // this is where the reversal happens
        table
            .insert(
                &CloudstateArrayItemKey {
                    namespace: key.namespace.clone(),
                    id: key.id.clone(),
                    index: key.index,
                },
                CloudstateArrayItemValue { data: value },
            )
            .unwrap();
    }
}

#[op2]
#[to_v8]
fn op_cloudstate_array_pop(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] array_id: String,
) -> Result<CloudstatePrimitiveData, Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();
    let mut table = write_txn.open_table(ARRAYS_TABLE).unwrap();

    let keys: Vec<CloudstateArrayItemKey> = table
        .iter()
        .unwrap()
        .map(|entry| entry.unwrap().0.value())
        .filter(|key| key.id == array_id && key.namespace == namespace)
        .collect();

    let length = keys.len() as i32;
    if length == 0 {
        return Ok(CloudstatePrimitiveData::Undefined);
    }

    let key = match keys.iter().find(|key| key.index == length - 1) {
        Some(key) => key,
        None => return Ok(CloudstatePrimitiveData::Undefined),
    };

    let value = table.remove(key).unwrap().unwrap().value().data;

    Ok(value)
}

#[op2]
#[to_v8]
fn op_cloudstate_array_shift(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] array_id: String,
) -> CloudstatePrimitiveData {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();

    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(ARRAYS_TABLE).unwrap();

    let keys: Vec<CloudstateArrayItemKey> = table
        .iter()
        .unwrap()
        .map(|entry| entry.unwrap().0.value())
        .filter(|key| key.id == array_id && key.namespace == namespace)
        .collect();

    let mut return_value = None;
    for key in keys {
        let value = table.remove(&key).unwrap();
        let value = value.unwrap().value().data;
        if (key.index - 1) >= 0 {
            table
                .insert(
                    &CloudstateArrayItemKey {
                        namespace: key.namespace.clone(),
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

#[op2]
#[to_v8]
fn op_cloudstate_cloudstate_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
) -> CloudstatePrimitiveData {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(OBJECTS_TABLE).unwrap();

    let result = table.iter().unwrap().find(|value| {
        value
            .as_ref()
            .unwrap()
            .1
            .value()
            .data
            .fields
            .get(&"id".to_string())
            .map_or_else(
                || false,
                |id_value| id_value == &CloudstatePrimitiveData::String(id.clone()),
            )
    });

    result
        .map(|result| {
            CloudstatePrimitiveData::ObjectReference(ObjectReference {
                id: result.unwrap().0.value().id.clone(),
            })
        })
        .unwrap()
}

#[op2]
fn op_cloudstate_map_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
    #[from_v8] value: CloudstatePrimitiveData,
) -> Result<(), Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

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

#[op2(fast)]
fn op_cloudstate_map_delete(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] map_id: String,
    #[string] key: String,
) -> bool {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(MAPS_TABLE).unwrap();

    let key = CloudstateMapFieldKey {
        namespace: namespace.to_string(),
        id: map_id.to_string(),
        field: key.to_string(),
    };

    let did_exist = match table.remove(&key).unwrap_or(None) {
        Some(_) => true,
        None => false,
    };
    did_exist
}

#[op2(fast)]
fn op_cloudstate_map_clear(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] map_id: String,
) {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(MAPS_TABLE).unwrap();

    let keys: Vec<CloudstateMapFieldKey> = table
        .iter()
        .unwrap()
        .map(|entry| entry.unwrap().0.value())
        .filter(|key| key.id == map_id && key.namespace == namespace)
        .collect();

    for key in keys {
        table.remove(&key).unwrap();
    }
}

#[op2]
#[to_v8]
fn op_cloudstate_map_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
) -> CloudstatePrimitiveData {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(MAPS_TABLE).unwrap();

    let key = CloudstateMapFieldKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
        field: field.to_string(),
    };

    let out = match table.get(key).unwrap_or(None) {
        Some(value) => value.value().data,
        None => CloudstatePrimitiveData::Undefined,
    };
    out
}

#[op2]
#[to_v8]
fn op_cloudstate_map_has(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
) -> CloudstatePrimitiveData {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(MAPS_TABLE).unwrap();

    let key = CloudstateMapFieldKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
        field: field.to_string(),
    };

    let out = match table.get(key).unwrap_or(None) {
        Some(_) => CloudstatePrimitiveData::Boolean(true),
        None => CloudstatePrimitiveData::Boolean(false),
    };
    out
}

#[op2]
fn op_cloudstate_array_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    index: i32,
    #[from_v8] value: CloudstatePrimitiveData,
) -> Result<(), Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(ARRAYS_TABLE).unwrap();

    let key = CloudstateArrayItemKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
        index: index,
    };

    let _ = table
        .insert(&key, CloudstateArrayItemValue { data: value })
        .unwrap();
    Ok(())
}

#[op2(fast)]
fn op_cloudstate_array_length(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] id: String,
) -> i32 {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(ARRAYS_TABLE).unwrap();

    let count = table
        .iter()
        .unwrap()
        .filter(|entry| entry.as_ref().unwrap().0.value().id == id)
        .count();

    count as i32
}

#[op2]
#[to_v8]
fn op_cloudstate_array_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    index: i32,
) -> CloudstatePrimitiveData {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(ARRAYS_TABLE).unwrap();

    let key = CloudstateArrayItemKey {
        namespace: namespace.to_string(),
        id: id.to_string(),
        index: index,
    };

    let result = table.get(key).unwrap();
    let result = result.map(|s| s.value().data);

    result.unwrap()
}

#[op2(fast)]
// #[to_v8]
fn op_cloudstate_map_size(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] map_id: String,
) -> Result<i32, Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();
    let table = read_txn.open_table(MAPS_TABLE).unwrap();

    let count = table
        .iter()
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|(key, _value)| key.value().id == map_id)
        .count();

    Ok(count as i32)
}

#[op2]
#[string]
fn op_cloudstate_object_root_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] alias: String,
) -> Result<Option<String>, Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let cs = cs.lock().unwrap();

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
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

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
fn op_cloudstate_create_transaction(
    state: &mut OpState,
    #[string] id: String,
) -> Result<(), Error> {
    println!("Creating transaction");
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();

    println!("Opening write transaction");
    let write_txn = cs.db.begin_write().unwrap();
    // cs.transactions.ins
    cs.transactions.insert(id, write_txn);
    Ok(())
}

#[op2(fast)]
fn op_cloudstate_commit_transaction(
    state: &mut OpState,
    #[string] id: String,
) -> Result<(), Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();
    let mut cs = cs.lock().unwrap();
    let write_txn = cs.transactions.remove(&id).unwrap();
    write_txn.commit().unwrap();
    Ok(())
}

//TODO: Lazy iterator?
#[op2]
#[to_v8]
fn op_cloudstate_map_values(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] map_id: String,
) -> Result<CloudstatePrimitiveDataVec, Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();

    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();

    let table = read_txn.open_table(MAPS_TABLE).unwrap();

    let mut values = vec![];

    for entry in table.iter().unwrap() {
        let (key, value) = entry.unwrap();
        println!("Key: {:?}", key.value());
        println!("Value: {:?}", value.value());
        if key.value().id == map_id {
            values.push(value.value().data);
        }
    }

    Ok(values.into())
}

#[op2]
#[to_v8]
fn op_cloudstate_array_sort<'a>(
    state: &mut OpState,
    scope: &mut v8::HandleScope<'a>,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] array_id: String,
    compare_fn: Option<&v8::Function>,
) -> CloudstatePrimitiveData {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();

    let mut cs = cs.lock().unwrap();

    let write_txn = cs.transactions.get_mut(&transaction_id).unwrap();

    let mut table = write_txn.open_table(ARRAYS_TABLE).unwrap();

    //TODO: Pull global scope instead of using empty one
    let object = v8::Object::new(scope);

    let mut compare_fn: Box<dyn FnMut(&CloudstatePrimitiveData, &CloudstatePrimitiveData) -> bool> =
        match compare_fn {
            Some(compare_fn) => {
                Box::new(|a: &CloudstatePrimitiveData, b: &CloudstatePrimitiveData| {
                    let a = a.clone().to_v8(scope).unwrap();
                    let b = b.clone().to_v8(scope).unwrap();

                    // hydrate them

                    // run compare_fn
                    let args = [a, b];

                    let result = compare_fn.call(scope, object.into(), &args).unwrap();

                    let result = result.to_boolean(scope);

                    return result.boolean_value(scope);
                })
            }
            None => Box::new(|a: &CloudstatePrimitiveData, b: &CloudstatePrimitiveData| {
                // default compare_fn
                let a = a.clone().to_v8(scope).unwrap();
                let b = b.clone().to_v8(scope).unwrap();

                a.uint32_value(scope) < b.uint32_value(scope)
            }),
        };

    let keys: Vec<CloudstateArrayItemKey> = table
        .iter()
        .unwrap()
        .map(|entry| entry.unwrap().0.value())
        .filter(|key| key.id == array_id && key.namespace == namespace)
        .collect();

    let mut values = vec![];

    for key in &keys {
        let value = table.get(key).unwrap().unwrap().value().data;
        values.push(value);
    }

    values.sort_by(|a, b| {
        if compare_fn(a, b) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    for (i, key) in keys.iter().enumerate() {
        let value = values[i].clone();
        table
            .insert(
                &CloudstateArrayItemKey {
                    namespace: key.namespace.clone(),
                    id: key.id.clone(),
                    index: key.index,
                },
                CloudstateArrayItemValue { data: value },
            )
            .unwrap();
    }

    return CloudstatePrimitiveData::Undefined;
}

#[op2]
#[to_v8]
fn op_cloudstate_map_keys(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] map_id: String,
) -> Result<CloudstatePrimitiveDataVec, Error> {
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();

    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(transaction_id.as_str()).unwrap();

    let table = read_txn.open_table(MAPS_TABLE).unwrap();

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

#[op2]
#[to_v8]
fn op_cloudstate_map_entries(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] map_id: String,
) -> Result<CloudstateEntriesVec, Error> {
    println!("Getting map entries");
    let cs = state
        .try_borrow_mut::<Arc<Mutex<ReDBCloudstate>>>()
        .unwrap();

    let cs = cs.lock().unwrap();

    let read_txn = cs.transactions.get(&transaction_id).unwrap();

    let table = read_txn.open_table(MAPS_TABLE).unwrap();

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
    pub constructor_name: Option<String>,
}

impl ToV8<'_> for CloudstateObjectData {
    fn to_v8<'a>(
        self,
        scope: &mut v8::HandleScope<'a>,
    ) -> Result<deno_core::v8::Local<'a, deno_core::v8::Value>, JsError> {
        let object = v8::Object::new(scope);
        for (key, value) in self.fields.iter() {
            let key = v8::Local::<v8::Value>::from(v8::String::new(scope, key).unwrap());
            let value = value.clone().to_v8(scope).unwrap();
            object.set(scope, key, value);
        }

        if let Some(constructor_name) = &self.constructor_name {
            // todo: we shouldn't be passing data through abnormal channels like this
            let constructor_name_key =
                v8::String::new(scope, "__cloudstate__constructorName").unwrap();
            let constructor_name = v8::String::new(scope, constructor_name).unwrap();
            object.set(scope, constructor_name_key.into(), constructor_name.into());
        }

        Ok(v8::Local::<v8::Value>::from(object))
    }

    type Error = JsError;
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

        Ok(CloudstateObjectData {
            fields,
            constructor_name: match object
                .get_constructor_name()
                .to_rust_string_lossy(scope)
                .as_str()
            {
                "Object" => None,
                constructor_name => Some(constructor_name.to_string()),
            },
        })
    }

    type Error = JsError;
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
    URL(Url),
    // Error(JsError),
    ObjectReference(ObjectReference),
    MapReference(String),
    ArrayReference(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ObjectReference {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
    ) -> Result<v8::Local<'a, v8::Value>, JsError> {
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

    type Error = JsError;
}

impl ToV8<'_> for CloudstatePrimitiveDataVec {
    type Error = JsError;

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
    ) -> Result<v8::Local<'a, v8::Value>, JsError> {
        Ok(v8::Local::<v8::Value>::from(match self {
            CloudstatePrimitiveData::Date(value) => deno_core::v8::Local::<v8::Value>::from(
                v8::Date::new(scope, value.timestamp_millis() as f64).unwrap(),
            ),
            CloudstatePrimitiveData::Number(value) => v8::Number::new(scope, value).into(),
            CloudstatePrimitiveData::String(value) => {
                v8::String::new(scope, &value).unwrap().into()
            }
            CloudstatePrimitiveData::Boolean(value) => v8::Boolean::new(scope, value).into(),
            CloudstatePrimitiveData::BigInt(value) => {
                v8::BigInt::new_from_words(scope, false, &value)
                    .unwrap()
                    .into()
            }
            CloudstatePrimitiveData::Undefined => v8::undefined(scope).into(),
            CloudstatePrimitiveData::Null => v8::null(scope).into(),
            CloudstatePrimitiveData::URL(value) => {
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
                    global
                        .get(scope, export_name)
                        .expect("CloudstateObjectReference class should be globally defined")
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
        }))
    }

    type Error = JsError;
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
            let constructor = object.get_constructor_name().to_rust_string_lossy(scope);
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
                        id: v8::Local::<v8::String>::try_from(object.get(scope, key).unwrap())
                            .unwrap()
                            .to_rust_string_lossy(scope),
                    }));
                }
                "CloudstateObjectReference" => {
                    let object_key = v8::String::new(scope, "objectId").unwrap().into();
                    let constructor_name_key = v8::String::new(scope, "constructorName").unwrap();

                    let constructor_name = object.get(scope, constructor_name_key.into()).unwrap();
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
            Err(JsError::from_v8_exception(scope, msg))
        }
    }

    type Error = JsError;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CloudstateMapFieldKey {
    pub namespace: String,
    pub id: String,
    pub field: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudstateMapFieldValue {
    pub data: CloudstatePrimitiveData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CloudstateArrayItemKey {
    pub namespace: String,
    pub id: String,
    pub index: i32,
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
    op_cloudstate_array_sort,
    op_cloudstate_cloudstate_get,
    op_cloudstate_commit_transaction,
    op_cloudstate_create_transaction,
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
    op_cloudstate_object_set
  ],
  esm_entry_point = "ext:cloudstate/cloudstate.js",
  esm = [ dir "src/extensions", "cloudstate.js" ],
);
