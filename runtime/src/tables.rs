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
