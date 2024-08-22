use crate::extensions::cloudstate::CloudstatePrimitiveData;
use crate::{
    extensions::cloudstate::CloudstateObjectKey,
    tables::{OBJECTS_TABLE, ROOTS_TABLE},
};
use anyhow::anyhow;
use redb::{Database, ReadableTable};


pub fn mark_and_sweep(db: &mut Database) -> anyhow::Result<()> {
    let reachable = mark(db)?;
    sweep(db, reachable)
}

fn mark(db: &Database) -> anyhow::Result<Vec<CloudstateObjectKey>> {
    let read = db.begin_read();
    let read = match read {
        Ok(read) => read,
        Err(e) => return Err(anyhow!(e)),
    };
    let roots_table = match read.open_table(ROOTS_TABLE) {
        Ok(table) => table,
        Err(e) => return Err(anyhow!(e)),
    };

    let objects_table = match read.open_table(OBJECTS_TABLE) {
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
    let mut reachable: Vec<CloudstateObjectKey> = Vec::new();
    let mut stack: Vec<CloudstateObjectKey> = Vec::with_capacity(roots.len());

    for root in roots {
        stack.push(root);
    }

    while let Some(object_key) = stack.pop() {
        if reachable.contains(&object_key) {
            continue;
        }

        reachable.push(object_key.clone());

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
                },
                CloudstatePrimitiveData::MapReference(_) => todo!(),
                _ => {/* These don't have references so they don't need anything */}
            }
        }
    }



    Ok(reachable)
}

fn sweep(db: &mut Database, reachable: Vec<CloudstateObjectKey>) -> anyhow::Result<()> {
    let read = db.begin_read();
    let read = match read {
        Ok(read) => read,
        Err(e) => return Err(anyhow!(e)),
    };
    let write = db.begin_write();
    let write = match write {
        Ok(write) => write,
        Err(e) => return Err(anyhow!(e)),
    };

    let objects_table = match read.open_table(OBJECTS_TABLE) {
        Ok(table) => table,
        Err(e) => return Err(anyhow!(e)),
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

    for key in to_delete {
        // open table
        let mut table = write.open_table(OBJECTS_TABLE)?;
        // delete key
        table.remove(&key)?;


    }
    read.close()?;
    write.commit()?;
    Ok(())

}
