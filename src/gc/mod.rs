use std::collections::BTreeSet;

use crate::extensions::cloudstate::{CloudstateMapFieldKey, CloudstatePrimitiveData};
use crate::tables::MAPS_TABLE;
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
    Map(CloudstateMapFieldKey),
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

                        for (key, value) in object.value().data.fields {
                            match value {
                                // CloudstatePrimitiveData::URL(i)
                                CloudstatePrimitiveData::ObjectReference(obj_ref) => {
                                    stack.push(Pointer::Object(CloudstateObjectKey {
                                        id: obj_ref,
                                        namespace: object_key.namespace.clone(),
                                    }));
                                }
                                CloudstatePrimitiveData::MapReference(map_ref) => {
                                    stack.push(Pointer::Map(CloudstateMapFieldKey {
                                        id: map_ref,
                                        field: key,
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
                Pointer::Map(key) => {
                    if let Some(ref map_table) = map_table {
                        let map = match map_table.get(&key)? {
                            Some(map) => map,
                            None => continue,
                        };
                        match map.value().data {
                            CloudstatePrimitiveData::ObjectReference(reference) => {
                                stack.push(Pointer::Object(CloudstateObjectKey {
                                    id: reference,
                                    namespace: key.namespace.clone(),
                                }));
                            },
                            CloudstatePrimitiveData::MapReference(reference) => {
                                stack.push(Pointer::Map(CloudstateMapFieldKey {
                                    id: reference,
                                    field: key.field.clone(),
                                    namespace: key.namespace.clone(),
                                }));
                            },
                            CloudstatePrimitiveData::ArrayReference(_) => todo!(),
                            _ => {/*Irrelevant for marking */}
                        }
                    } else {
                        // This should never happen, but it def could 💀
                    }
                }
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
            Err(e) => return Err(anyhow!("test")),
        };
        let mut maps_table = match tx.open_table(MAPS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!("test")),
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
                if !reachable.contains(&Pointer::Map(key.clone())) {
                    to_delete.push(Pointer::Map(key));
                }
            }
        }

        for pointer in to_delete {
            // delete key
            let _: Result<(), ()> = match pointer {
                Pointer::Object(key) => {
                    let _ =objects_table.remove(&key)?;
                    Ok(())
                },
                Pointer::Map(key) => {
                    let _ = maps_table.remove(&key)?;
                    Ok(())
                },
            };
        }
    }
    tx.commit()?;
    Ok(())
}
