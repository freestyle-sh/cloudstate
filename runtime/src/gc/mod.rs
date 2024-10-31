use std::collections::BTreeSet;

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
use tracing::event;

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
    println!("mark");
    let reachable = {
        let roots_table = match tx.open_table(ROOTS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!(e)),
        };

        let mut roots: Vec<CloudstateObjectKey> = Vec::new();

        println!("roots_table: {:?}", roots_table.len());
        for item in roots_table.iter()? {
            if let Ok((key, root)) = item {
                let _key = key.value();
                let root = root.value();
                roots.push(CloudstateObjectKey { id: root.id });
            }
        }

        let objects_table = match tx.open_table(OBJECTS_TABLE) {
            Ok(table) => {
                println!("objects_table: {:?}", table.len());
                Some(table)
            }
            Err(_e) => None,
        };

        let map_table = match tx.open_table(MAPS_TABLE) {
            Ok(table) => {
                println!("map_table: {:?}", table.len());
                Some(table)
            }
            Err(_e) => None,
        };

        let arr_table = match tx.open_table(ARRAYS_TABLE) {
            Ok(table) => {
                println!("arr_table: {:?}", table.len());
                Some(table)
            }
            Err(_e) => None,
        };

        let mut reachable: BTreeSet<Pointer> = BTreeSet::new();

        let mut stack: Vec<Pointer> = Vec::with_capacity(roots.len());

        stack.extend(roots.iter().map(|root| Pointer::Object(root.clone())));

        while let Some(pointer) = stack.pop() {
            println!("stack: {:?}", stack.len());
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
                        for item in map_table.iter()? {
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
                        for item in arr_table.iter()? {
                            if let Ok((key, value)) = item {
                                let key = key.value();
                                let value = value.value();

                                if key.id == arr_ref.id {
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
                                        CloudstatePrimitiveData::ArrayReference(
                                            arr_ref_internal,
                                        ) => {
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
                }
                _ => { /* These don't have references so they don't need anything */ }
            }
        }
        reachable
    };
    tx.close()?;
    Ok(reachable)
}

/// Consumes the transaction and deletes all objects not in the set
fn sweep(tx: WriteTransaction, reachable: &BTreeSet<Pointer>) -> anyhow::Result<()> {
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

        println!("objects_table: {:?}", objects_table.len());
        for item in objects_table.iter()? {
            // println!("item: {:?}", item);
            if let Ok((key, _value)) = item {
                let key = key.value();
                if !reachable.contains(&Pointer::Object(key.clone())) {
                    to_delete.push(Pointer::Object(key));
                }
            }
        }

        println!("maps_table: {:?}", maps_table.len());
        for item in maps_table.iter()? {
            // println!("item: {:?}", item);
            if let Ok((key, _value)) = item {
                let key = key.value();

                if !reachable.contains(&Pointer::Map(CloudstateObjectKey { id: key.id.clone() })) {
                    to_delete.push(Pointer::MapField(key));
                }
            }
        }

        println!("arrays_table: {:?}", arrays_table.len());
        for item in arrays_table.iter()? {
            // println!("item: {:?}", item);
            if let Ok((key, _value)) = item {
                let key = key.value();

                if !reachable.contains(&Pointer::Array(CloudstateObjectKey { id: key.id.clone() }))
                {
                    to_delete.push(Pointer::ArrayItem(key));
                }
            }
        }

        println!("to_delete: {:?}", to_delete.len());
        for pointer in to_delete {
            // println!("pointer: {:?}", pointer);
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

// #[cfg(test)]
// mod test {
//     use redb::Database;

//     use crate::gc::mark_and_sweep;

//     #[test]
//     fn run_garbage_collector() {
//         println!("run_garbage_collector");
//         // let db = Database::open(
//         //     "/Users/jacobzwang/Documents/GitHub/cloudstate/cli/cloudstate-collected",
//         // )
//         // .unwrap();
//         // println!("db open");
//         // let _res = mark_and_sweep(&db);
//     }
// }
