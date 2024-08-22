use std::any;
use std::collections::{BTreeSet, HashSet};

use crate::extensions::cloudstate::{CloudstatePrimitiveData, ReDBCloudstate};
use crate::{
    extensions::cloudstate::CloudstateObjectKey,
    tables::{OBJECTS_TABLE, ROOTS_TABLE},
};
use anyhow::anyhow;
use redb::{Database, ReadTransaction, ReadableTable, WriteTransaction};

type ObjectsTable<'a> = redb::Table<
    'a,
    crate::bincode::Bincode<CloudstateObjectKey>,
    crate::bincode::Bincode<crate::extensions::cloudstate::CloudstateObjectValue>,
>;

pub fn mark_and_sweep(db: &Database) -> anyhow::Result<()> {
    let tx = db.begin_read()?;
    let reachable = mark(tx)?;

    let tx = db.begin_write()?;
    let _ = sweep(tx, reachable);
    Ok(())
}

fn mark(tx: ReadTransaction) -> anyhow::Result<BTreeSet<CloudstateObjectKey>> {
    let reachable = {
        let objects_table = match tx.open_table(OBJECTS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!(e)),
        };

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

        // let mut visited: Vec<String> = Vec::new();
        let mut reachable: BTreeSet<CloudstateObjectKey> = BTreeSet::new();
        let mut stack: Vec<CloudstateObjectKey> = Vec::with_capacity(roots.len());

        for root in roots {
            stack.push(root);
        }

        while let Some(object_key) = stack.pop() {
            if reachable.contains(&object_key) {
                continue;
            }

            reachable.insert(object_key.clone());

            let object = match objects_table.get(&object_key)? {
                Some(object) => object,
                None => continue,
            };

            for (_key, value) in object.value().data.fields {
                match value {
                    // CloudstatePrimitiveData::URL(i)
                    CloudstatePrimitiveData::ObjectReference(obj_reference) => {
                        stack.push(CloudstateObjectKey {
                            id: obj_reference,
                            namespace: object_key.namespace.clone(),
                        });
                    }
                    CloudstatePrimitiveData::MapReference(_) => todo!(),
                    _ => { /* These don't have references so they don't need anything */ }
                }
            }
        }
        reachable
    };
    tx.close()?;
    Ok(reachable)
}

fn sweep(tx: WriteTransaction, reachable: BTreeSet<CloudstateObjectKey>) -> anyhow::Result<()> {
    {
        let mut objects_table = match tx.open_table(OBJECTS_TABLE) {
            Ok(table) => table,
            Err(e) => return Err(anyhow!("test")),
        };
        let mut to_delete: Vec<CloudstateObjectKey> = Vec::new();

        for item in objects_table.iter()? {
            if let Ok((key, _value)) = item {
                let key = key.value();
                if !reachable.contains(&key) {
                    to_delete.push(key);
                }
            }
        }
        println!("TO DELETE: {:?}", to_delete);
        for key in to_delete {
            // open table
            
            // delete key
            objects_table.remove(&key)?;
        }
    }
    tx.commit()?;
    Ok(())
}
