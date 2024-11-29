use crate::extensions::cloudstate::{
    CloudstateArrayItemKey, CloudstateMapFieldKey, CloudstatePrimitiveData,
};
use crate::tables::{ARRAYS_TABLE, MAPS_TABLE};
use crate::{
    extensions::cloudstate::CloudstateObjectKey,
    tables::{OBJECTS_TABLE, ROOTS_TABLE},
};
use anyhow::anyhow;
use redb::{Database, ReadTransaction, ReadableTable, ReadableTableMetadata, WriteTransaction};
use std::collections::BTreeSet;
use tracing::{debug, instrument};
use tracing::{event, info};

#[instrument(skip(db))]
pub fn mark_and_sweep(db: &Database) -> anyhow::Result<&Database> {
    let tx = db.begin_read()?;
    let reachable = mark(tx)?;

    let tx = match db.begin_write() {
        Ok(out) => out,
        Err(e) => {
            event!(
                tracing::Level::ERROR,
                "Error creating write transaction: {}",
                e
            );

            panic!("Error creating write transaction: {}", e)
        }
    };

    let _ = sweep(tx, &reachable);
    Ok(db)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pointer {
    Object(CloudstateObjectKey),
    Map(CloudstateObjectKey),
    Array(CloudstateObjectKey),
    // These are only deletable but not sweepable
    MapField(CloudstateMapFieldKey),
    ArrayItem(CloudstateArrayItemKey),
}

/// Comsumes the transaction and returns a set of reachable objects
#[instrument(skip(tx))]
pub fn mark(tx: ReadTransaction) -> anyhow::Result<BTreeSet<Pointer>> {
    let reachable = {
        let roots_table = match tx.open_table(ROOTS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!(e)),
        };

        let mut roots: Vec<CloudstateObjectKey> = Vec::new();

        info!("{} items in roots table", roots_table.len().unwrap_or(0));
        for item in roots_table.iter()? {
            if let Ok((key, root)) = item {
                let _key = key.value();
                let root = root.value();
                roots.push(CloudstateObjectKey { id: root.id });
            }
        }

        let objects_table = match tx.open_table(OBJECTS_TABLE) {
            Ok(table) => {
                info!("{} items in objects table", table.len().unwrap_or(0));
                Some(table)
            }
            Err(_e) => None,
        };

        let map_table = match tx.open_table(MAPS_TABLE) {
            Ok(table) => {
                info!("{} items in map table", table.len().unwrap_or(0));
                Some(table)
            }
            Err(_e) => None,
        };

        let arr_table = match tx.open_table(ARRAYS_TABLE) {
            Ok(table) => {
                info!("{} items in array table", table.len().unwrap_or(0));
                Some(table)
            }
            Err(_e) => None,
        };

        let mut reachable: BTreeSet<Pointer> = BTreeSet::new();

        let mut stack: Vec<Pointer> = Vec::with_capacity(roots.len());

        stack.extend(roots.iter().map(|root| Pointer::Object(root.clone())));

        while let Some(pointer) = stack.pop() {
            debug!("Stack size: {}", stack.len());
            debug!("Pointer: {:?}", pointer);
            if reachable.contains(&pointer) {
                continue;
            }

            reachable.insert(pointer.clone());

            match pointer {
                Pointer::Object(object_key) => {
                    if let Some(ref objects_table) = objects_table {
                        let object = match objects_table.get(&object_key)? {
                            Some(object) => object,
                            None => continue,
                        };

                        for (_key, value) in object.value().data.fields {
                            match value {
                                CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                                    stack.push(Pointer::Object(CloudstateObjectKey {
                                        id: obj_ref.id,
                                    }));
                                }
                                CloudstatePrimitiveData::MapReference(map_ref) => {
                                    stack.push(Pointer::Map(CloudstateObjectKey { id: map_ref }));
                                }
                                CloudstatePrimitiveData::ArrayReference(arr_ref) => {
                                    stack.push(Pointer::Array(CloudstateObjectKey { id: arr_ref }));
                                }
                                _ => {
                                    /* These don't have references so they don't need anything */
                                }
                            }
                        }
                    }
                }
                Pointer::Map(map_reference) => {
                    if let Some(ref map_table) = map_table {
                        for item in map_table.range(
                            CloudstateMapFieldKey {
                                id: map_reference.id.clone(),
                                field: String::new(),
                            }..CloudstateMapFieldKey {
                                id: map_reference.id.clone() + "\u{0}", // null character so it goes through all the fields
                                field: String::new(),
                            },
                        )? {
                            if let Ok((key, value)) = item {
                                let key = key.value();
                                let value = value.value();
                                if key.id == map_reference.id {
                                    match value.data {
                                        CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                                            stack.push(Pointer::Object(CloudstateObjectKey {
                                                id: obj_ref.id,
                                            }));
                                        }
                                        CloudstatePrimitiveData::MapReference(map_ref_internal) => {
                                            stack.push(Pointer::Map(CloudstateObjectKey {
                                                id: map_ref_internal,
                                            }));
                                        }
                                        CloudstatePrimitiveData::ArrayReference(arr_ref) => {
                                            stack.push(Pointer::Array(CloudstateObjectKey {
                                                id: arr_ref,
                                            }));
                                        }
                                        _ => {
                                            /* These don't have references so they don't need anything */
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Pointer::Array(arr_ref) => {
                    if let Some(ref arr_table) = arr_table {
                        for item in arr_table.range(
                            CloudstateArrayItemKey {
                                id: arr_ref.id.clone(),
                                index: 0,
                            }..CloudstateArrayItemKey {
                                id: arr_ref.id.clone(),
                                index: i32::MAX,
                            },
                        )? {
                            if let Ok((_key, value)) = item {
                                let value = value.value();

                                match value.data {
                                    CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                                        stack.push(Pointer::Object(CloudstateObjectKey {
                                            id: obj_ref.id,
                                        }));
                                    }
                                    CloudstatePrimitiveData::MapReference(map_ref) => {
                                        stack.push(Pointer::Map(CloudstateObjectKey {
                                            id: map_ref,
                                        }));
                                    }
                                    CloudstatePrimitiveData::ArrayReference(arr_ref_internal) => {
                                        stack.push(Pointer::Array(CloudstateObjectKey {
                                            id: arr_ref_internal,
                                        }));
                                    }
                                    _ => {
                                        /* These don't have references so they don't need anything */
                                    }
                                }
                            }
                        }
                    }
                }
                _ => { /* These don't have references so they don't need anything */ }
            }
        }
        info!("Found {} reachable objects", reachable.len());

        reachable
    };

    tx.close()?;

    Ok(reachable)
}

/// Consumes the transaction and deletes all objects not in the set
#[instrument(skip(tx, reachable))]
pub fn sweep(tx: WriteTransaction, reachable: &BTreeSet<Pointer>) -> anyhow::Result<()> {
    {
        let mut objects_table = match tx.open_table(OBJECTS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!(e)),
        };
        let mut maps_table = match tx.open_table(MAPS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!(e)),
        };

        let mut arrays_table = match tx.open_table(ARRAYS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!(e)),
        };

        let mut to_delete: Vec<Pointer> = Vec::new();

        for item in objects_table.iter()? {
            if let Ok((key, _value)) = item {
                let key = key.value();
                if !reachable.contains(&Pointer::Object(key.clone())) {
                    to_delete.push(Pointer::Object(key));
                }
            }
        }

        for item in maps_table.iter()? {
            if let Ok((key, _value)) = item {
                let key = key.value();

                if !reachable.contains(&Pointer::Map(CloudstateObjectKey { id: key.id.clone() })) {
                    to_delete.push(Pointer::MapField(key));
                }
            }
        }

        for item in arrays_table.iter()? {
            if let Ok((key, _value)) = item {
                let key = key.value();

                if !reachable.contains(&Pointer::Array(CloudstateObjectKey { id: key.id.clone() }))
                {
                    to_delete.push(Pointer::ArrayItem(key));
                }
            }
        }

        info!("deleting {:?} items", to_delete.len());
        for pointer in to_delete {
            // delete key
            let _: anyhow::Result<()> = match pointer {
                Pointer::Object(key) => {
                    let _ = objects_table.remove(&key)?;
                    Ok(())
                }

                Pointer::MapField(field) => {
                    let _ = maps_table.remove(&field)?;
                    Ok(())
                }
                Pointer::ArrayItem(field) => {
                    let _ = arrays_table.remove(&field)?;
                    Ok(())
                }
                _ => {
                    // Array and Map don't exist without their reference or their items, so they don't need to be garbage collected
                    Ok(())
                }
            };
        }
    }
    tx.commit()?;
    Ok(())
}
