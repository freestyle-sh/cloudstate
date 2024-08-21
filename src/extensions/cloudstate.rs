use crate::bincode::Bincode;
use deno_core::anyhow::Error;
use deno_core::error::JsError;
use deno_core::*;
use redb::ReadableTable;
use redb::{Database, TableDefinition, WriteTransaction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use v8::GetPropertyNamesArgs;

#[op2]
fn op_cloudstate_object_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[from_v8] value: CloudstateObjectData,
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
#[to_v8]
fn op_cloudstate_object_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
) -> Result<CloudstateObjectData, Error> {
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
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
#[to_v8]
fn op_cloudstate_get_test_object<'a>(state: &mut OpState) -> CloudstateObjectData {
    return CloudstateObjectData {
        fields: HashMap::from([
            ("test".to_string(), CloudstatePrimitiveData::Number(1.0)),
            ("test2".to_string(), CloudstatePrimitiveData::Number(2.0)),
        ]),
    };
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
            // let key = key.to_rust_string_lossy(scope);
            let value = object.get(scope, key).unwrap();
            let value = CloudstatePrimitiveData::from_v8(scope, value).unwrap();
            fields.insert(key.to_rust_string_lossy(scope), value);
        }
        Ok(CloudstateObjectData { fields })
    }

    type Error = JsError;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CloudstatePrimitiveData {
    Number(f64),
    String(String),
    Boolean(bool),
    BigInt(i64),
    Undefined,
    Null,
    // Date(DateTime<Utc>),
    // RegExp(String),
    URL(Url),
    // Error(JsError),
    ObjectReference(String),
    MapReference(String),
}

impl ToV8<'_> for CloudstatePrimitiveData {
    fn to_v8<'a>(
        self,
        scope: &mut v8::HandleScope<'a>,
    ) -> Result<v8::Local<'a, v8::Value>, JsError> {
        Ok(v8::Local::<v8::Value>::from(match self {
            CloudstatePrimitiveData::Number(value) => v8::Number::new(scope, value).into(),
            CloudstatePrimitiveData::String(value) => {
                v8::String::new(scope, &value).unwrap().into()
            }
            CloudstatePrimitiveData::Boolean(value) => v8::Boolean::new(scope, value).into(),
            CloudstatePrimitiveData::BigInt(value) => {
                v8::BigInt::new_from_i64(scope, value.clone()).into()
            }
            CloudstatePrimitiveData::Undefined => v8::undefined(scope),
            CloudstatePrimitiveData::Null => v8::null(scope).into(),
            // CloudstatePrimitiveData::Date(value) => v8::Date::new(
            //     scope,
            //     value.timestamp_nanos_opt().unwrap() as f64 / 1_000_000.0,
            // )
            // .unwrap(),
            // CloudstatePrimitiveData::RegExp(value) => v8::RegExp::new(scope, value).unwrap().into(),
            CloudstatePrimitiveData::URL(value) => {
                v8::String::new(scope, value.as_str()).unwrap().into()
            }
            // CloudstatePrimitiveData::Error(value) => v8::Error::new(scope, value.clone()).into(),
            CloudstatePrimitiveData::ObjectReference(value) => {
                v8::String::new(scope, &value).unwrap().into()
            }
            CloudstatePrimitiveData::MapReference(value) => {
                v8::String::new(scope, &value).unwrap().into()
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
            let (value, _) = bigint.i64_value();
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
        } else {
            Ok(CloudstatePrimitiveData::Undefined)
        }
    }

    type Error = JsError;
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
