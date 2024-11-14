use crate::{
    bincode::Bincode,
    blob_storage::CloudstateBlobMetadata,
    extensions::cloudstate::{
        CloudstateArrayItemKey, CloudstateArrayItemValue, CloudstateBlobKey, CloudstateMapFieldKey,
        CloudstateMapFieldValue, CloudstateObjectKey, CloudstateObjectValue, CloudstateRootKey,
        CloudstateRootValue,
    },
};

use redb::{ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

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

// backup utilities here, so when we add/remove tables we can easily update the backup code

fn backup_table<K: redb::Key + 'static, V: redb::Value + 'static>(
    table_definition: redb::TableDefinition<K, V>,
    read: &ReadTransaction,
    write: &WriteTransaction,
) -> () {
    if let Ok(table) = read.open_table(table_definition) {
        let mut write_table = write.open_table(table_definition).unwrap();
        for item in table.iter().unwrap() {
            if let Ok((key, value)) = item {
                write_table.insert(key.value(), value.value()).unwrap();
            }
        }
    }
    // If table doesn't exist/doesn't have anything in it, it throws an error, so we pass over that
    ()
}

pub fn backup_all_tables(read: &ReadTransaction, write: &WriteTransaction) -> () {
    backup_table(ROOTS_TABLE, read, write);
    backup_table(OBJECTS_TABLE, read, write);
    backup_table(MAPS_TABLE, read, write);
    backup_table(ARRAYS_TABLE, read, write);
    backup_table(BLOBS_TABLE, read, write);
    ()
}
