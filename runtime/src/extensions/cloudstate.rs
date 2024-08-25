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

#[op2]
fn op_cloudstate_map_set(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
    #[from_v8] value: CloudstatePrimitiveData,
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
#[to_v8]
fn op_cloudstate_map_get(
    state: &mut OpState,
    #[string] transaction_id: String,
    #[string] namespace: String,
    #[string] id: String,
    #[string] field: String,
) -> CloudstatePrimitiveData {
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

    result.unwrap()
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
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
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
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
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
    let cs = state.try_borrow_mut::<ReDBCloudstate>().unwrap();
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
    // RegExp(String),
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
                "CloudstateObjectReference" => {
                    let object_key = v8::String::new(scope, "objectId").unwrap().into();
                    let constructor_name_key = v8::String::new(scope, "constructorName").unwrap();

                    let constructor_name = object.get(scope, constructor_name_key.into()).unwrap();
                    let constructor_name = if constructor_name.is_undefined() {
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
    op_cloudstate_object_set,
    op_cloudstate_object_get,
    op_cloudstate_object_root_set,
    op_cloudstate_object_root_get,
    op_cloudstate_map_set,
    op_cloudstate_map_get,
    op_create_transaction,
    op_commit_transaction,
    op_cloudstate_array_set,
    op_cloudstate_array_get,
    op_cloudstate_array_length,
  ],
  esm_entry_point = "ext:cloudstate/cloudstate.js",
  esm = [ dir "src/extensions", "cloudstate.js" ],
);