use std::collections::HashMap;

use crate::{
    bincode::Bincode,
    blob_storage::CloudstateBlobMetadata,
    extensions::cloudstate::{
        CloudstateArrayItemKey, CloudstateArrayItemValue, CloudstateBlobKey, CloudstateMapFieldKey,
        CloudstateMapFieldValue, CloudstateObjectKey, CloudstateObjectValue, CloudstateRootKey,
        CloudstateRootValue,
    },
};

use redb::{
    ReadTransaction, ReadableTable, ReadableTableMetadata, TableDefinition, TableHandle,
    WriteTransaction,
};

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

pub const ARRAYS_TABLE: TableDefinition<
    Bincode<CloudstateArrayItemKey>,
    Bincode<CloudstateArrayItemValue>,
> = TableDefinition::new("arrays");

pub const BLOBS_TABLE: TableDefinition<
    Bincode<CloudstateBlobKey>,
    Bincode<CloudstateBlobMetadata>,
> = TableDefinition::new("blobs");

impl<K: redb::Key, V: redb::Value> FreestyleTable for TableDefinition<'_, K, V> {
    fn backup(
        &self,
        read: &ReadTransaction,
        write: &WriteTransaction,
        handle: &mut ProgressHandle,
    ) -> anyhow::Result<()> {
        backup_table(*self, read, write, handle)
    }
}

pub struct ProgressHandle<'a> {
    progress_callback: Option<Box<dyn FnMut(String, TableBackupProgress) + 'a>>,
}

pub trait FreestyleTable {
    fn backup(
        &self,
        read: &ReadTransaction,
        write: &WriteTransaction,
        handle: &mut ProgressHandle,
        // progress_callback: &mut Option<Box<dyn FnMut(String, TableBackupProgress) + '_>>,
    ) -> anyhow::Result<()>;
}
// backup utilities here, so when we add/remove tables we can easily update the backup code

const FREESTYLE_TABLE_LIST: [&dyn FreestyleTable; 5] = [
    &ROOTS_TABLE,
    &OBJECTS_TABLE,
    &MAPS_TABLE,
    &ARRAYS_TABLE,
    &BLOBS_TABLE,
];

#[derive(Debug, Clone)]
pub struct TableBackupProgress {
    pub total: u64,
    pub current: u64,
}

#[derive(Debug, Clone)]
pub struct BackupProgress {
    pub tables: Vec<(String, TableBackupProgress)>,
}

fn backup_table<K: redb::Key + 'static, V: redb::Value + 'static>(
    table_definition: redb::TableDefinition<K, V>,
    read: &ReadTransaction,
    write: &WriteTransaction,
    handle: &mut ProgressHandle,
    // progress_callback: &mut Option<Box<dyn FnMut(String, TableBackupProgress) + '_>>,
) -> anyhow::Result<()> {
    if let Ok(table) = read.open_table(table_definition) {
        let table_length = match table.len() {
            Ok(len) => len,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Failed to get table length for table: {:?}",
                    table_definition.name()
                ))
            }
        };
        let mut write_table = match write.open_table(table_definition) {
            Ok(table) => table,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Failed to open table for writing: {:?}",
                    table_definition.name()
                ))
            }
        };

        let table_iterator = match table.iter() {
            Ok(iter) => iter,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Failed to get table iterator for table: {:?}",
                    table_definition.name()
                ))
            }
        };

        for (index, item) in table_iterator.enumerate() {
            if let Ok((key, value)) = item {
                write_table.insert(key.value(), value.value()).unwrap();
            }
            if let Some(ref mut callback) = handle.progress_callback {
                callback(
                    table_definition.name().to_string(),
                    TableBackupProgress {
                        total: table_length,
                        current: index as u64 + 1,
                    },
                );
            }
        }
    }
    Ok(())
}

pub fn backup_all_tables<'a>(
    read: &ReadTransaction,
    write: &WriteTransaction,
    progress_callback: &mut Option<Box<dyn FnMut(BackupProgress) -> () + 'a>>,
) -> anyhow::Result<()> {
    let mut backup_progress = BackupProgress { tables: Vec::new() };
    let mut handle = ProgressHandle {
        progress_callback: Some(Box::new(|table_name: String, prog| {
            // println!("Backing up table: {:?} {:?}", table_name, prog);
            if let Some(ref mut table) = backup_progress
                .tables
                .iter_mut()
                .find(|(name, _)| name == &table_name)
            {
                table.1 = prog;
            } else {
                backup_progress.tables.push((table_name, prog));
            }

            // backup_progress.tables.insert(table_name, prog);
            if let Some(callback) = progress_callback {
                callback(backup_progress.clone());
            }
        })),
    };

    for table in FREESTYLE_TABLE_LIST {
        table.backup(read, write, &mut handle)?;
    }
    Ok(())
}
