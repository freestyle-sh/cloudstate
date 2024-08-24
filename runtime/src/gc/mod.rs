use std::collections::BTreeSet;

use crate::extensions::cloudstate::{CloudstateArrayItemKey, CloudstateMapFieldKey, CloudstatePrimitiveData};
use crate::tables::{ARRAYS_TABLE, MAPS_TABLE};
use crate::{
    extensions::cloudstate::CloudstateObjectKey,
    tables::{OBJECTS_TABLE, ROOTS_TABLE},
};
use anyhow::anyhow;
use redb::{Database, ReadTransaction, ReadableTable, WriteTransaction};

pub fn mark_and_sweep(db: &Database) -> anyhow::Result<()> {
    let tx = db.begin_read()?;
    let reachable = mark(tx)?;

    let tx = db.begin_write()?;
    let _ = sweep(tx, &reachable);
    Ok(())
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Pointer {
    Object(CloudstateObjectKey),
    Map(CloudstateObjectKey),
    Array(CloudstateObjectKey),
    // These are only deletable but not sweepable
    MapField(CloudstateMapFieldKey),
    ArrayItem(CloudstateArrayItemKey),
}



/// Comsumes the transaction and returns a set of reachable objects
fn mark(tx: ReadTransaction) -> anyhow::Result<BTreeSet<Pointer>> {
    let reachable = {
        let roots_table = match tx.open_table(ROOTS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!(e)),
        };

        let mut roots: Vec<CloudstateObjectKey> = Vec::new();

        for item in roots_table.iter()? {
            if let Ok((key, root)) = item {
                let key = key.value();
                let root = root.value();
                roots.push(CloudstateObjectKey {
                    id: root.id,
                    namespace: key.namespace,
                });
            }
        }

        let objects_table = match tx.open_table(OBJECTS_TABLE) {
            Ok(table) => Some(table),
            Err(_e) => None,
        };

        let map_table = match tx.open_table(MAPS_TABLE) {
            Ok(table) => Some(table),
            Err(_e) => None,
        };

        let arr_table = match tx.open_table(ARRAYS_TABLE) {
            Ok(table) => Some(table),
            Err(_e) => None,
        };

        let mut reachable: BTreeSet<Pointer> = BTreeSet::new();

        let mut stack: Vec<Pointer> = Vec::with_capacity(roots.len());

        stack.extend(roots.iter().map(|root| Pointer::Object(root.clone())));

        while let Some(pointer) = stack.pop() {
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
                                        namespace: object_key.namespace.clone(),
                                    }));
                                }
                                CloudstatePrimitiveData::MapReference(map_ref) => {
                                    stack.push(Pointer::Map(CloudstateObjectKey {
                                        id: map_ref,
                                        namespace: object_key.namespace.clone(),
                                    }));
                                }
                                CloudstatePrimitiveData::ArrayReference(arr_ref) => {
                                    stack.push(Pointer::Array(CloudstateObjectKey {
                                        id: arr_ref,
                                        namespace: object_key.namespace.clone(),
                                    }));
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
                        for item in map_table.iter()? {
                            if let Ok((key, value)) = item {
                                let key = key.value();
                                let value = value.value();
                                if key.id == map_reference.id
                                    && key.namespace == map_reference.namespace
                                {
                                    match value.data {
                                        CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                                            stack.push(Pointer::Object(CloudstateObjectKey {
                                                id: obj_ref.id,
                                                namespace: map_reference.namespace.clone(),
                                            }));
                                        }
                                        CloudstatePrimitiveData::MapReference(map_ref_internal) => {
                                            stack.push(Pointer::Map(CloudstateObjectKey {
                                                id: map_ref_internal,
                                                namespace: map_reference.namespace.clone(),
                                            }));
                                        }
                                        CloudstatePrimitiveData::ArrayReference(arr_ref) => {
                                            stack.push(Pointer::Array(CloudstateObjectKey {
                                                id: arr_ref,
                                                namespace: map_reference.namespace.clone(),
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
                        for item in arr_table.iter()? {
                            if let Ok((key, value)) = item {
                                let key = key.value();
                                let value = value.value();
                            
                                if key.id == arr_ref.id && key.namespace == arr_ref.namespace {
                                    match value.data {
                                        CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                                            stack.push(Pointer::Object(CloudstateObjectKey {
                                                id: obj_ref.id,
                                                namespace: arr_ref.namespace.clone(),
                                            }));
                                        }
                                        CloudstatePrimitiveData::MapReference(map_ref) => {
                                            stack.push(Pointer::Map(CloudstateObjectKey {
                                                id: map_ref,
                                                namespace: arr_ref.namespace.clone(),
                                            }));
                                        }
                                        CloudstatePrimitiveData::ArrayReference(
                                            arr_ref_internal,
                                        ) => {
                                            stack.push(Pointer::Array(CloudstateObjectKey {
                                                id: arr_ref_internal,
                                                namespace: arr_ref.namespace.clone(),
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
                },
                _ => {
                    /* These don't have references so they don't need anything */
                }
                // Pointer::MapField(map_field_pointer) => {
                //     if let Some(ref map_table) = map_table {
                //         if let Ok(Some(value)) = map_table.get(&map_field_pointer) {
                //             match value.value().data {
                //                 CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                //                     stack.push(Pointer::Object(CloudstateObjectKey {
                //                         id: obj_ref.id,
                //                         namespace: map_field_pointer.namespace.clone(),
                //                     }));
                //                 }
                //                 CloudstatePrimitiveData::MapReference(map_ref_internal) => {
                //                     stack.push(Pointer::Map(CloudstateObjectKey {
                //                         id: map_ref_internal,
                //                         namespace: map_field_pointer.namespace.clone(),
                //                     }));
                //                 }
                //                 CloudstatePrimitiveData::ArrayReference(arr_ref) => {
                //                     stack.push(Pointer::Array(CloudstateObjectKey {
                //                         id: arr_ref,
                //                         namespace: map_field_pointer.namespace.clone(),
                //                     }));
                //                 }
                //                 _ => {
                //                     /* These don't have references so they don't need anything */
                //                 }
                //             }
                //         }
                //     }
                // }
                // Pointer::ArrayItem(arr_item) => {
                //     if let Some(ref arr_table) = arr_table {
                //         if let Ok(Some(value)) = arr_table.get(&arr_item) {
                //             match value.value().data {
                //                 CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                //                     stack.push(Pointer::Object(CloudstateObjectKey {
                //                         id: obj_ref.id,
                //                         namespace: arr_item.namespace.clone(),
                //                     }));
                //                 }
                //                 CloudstatePrimitiveData::MapReference(map_ref_internal) => {
                //                     stack.push(Pointer::Map(CloudstateObjectKey {
                //                         id: map_ref_internal,
                //                         namespace: arr_item.namespace.clone(),
                //                     }));
                //                 }
                //                 CloudstatePrimitiveData::ArrayReference(arr_ref) => {
                //                     stack.push(Pointer::Array(CloudstateObjectKey {
                //                         id: arr_ref,
                //                         namespace: arr_item.namespace.clone(),
                //                     }));
                //                 }
                //                 _ => {
                //                     /* These don't have references so they don't need anything */
                //                 }
                //             }
                //         }
                //     }
                // }
            }
        }
        reachable
    };
    tx.close()?;
    Ok(reachable)
}

/// Consumes the transaction and deletes all objects not in the set
fn sweep(tx: WriteTransaction, reachable: &BTreeSet<Pointer>) -> anyhow::Result<()> {
    println!("SWEEPING WITH REACHABLE: {:?}", reachable);
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

                if !reachable.contains(&Pointer::Map(
                    CloudstateObjectKey {
                        id: key.id.clone(),
                        namespace: key.namespace.clone(),
                    },
                )) {
                    to_delete.push(Pointer::MapField(key));
                }
            }
        }

        for item in arrays_table.iter()? {
            if let Ok((key, _value)) = item {
                let key = key.value();

                if !reachable.contains(&Pointer::Array(
                    CloudstateObjectKey {
                        id: key.id.clone(),
                        namespace: key.namespace.clone(),
                    },
                )) {
                    to_delete.push(Pointer::ArrayItem(key));
                }
            }
        }

        

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
                },
                Pointer::ArrayItem(field) => {
                    let _ = arrays_table.remove(&field)?;
                    Ok(())
                },
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
