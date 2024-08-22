use crate::{
    bincode::Bincode,
    extensions::cloudstate::{
        CloudstateMapFieldKey, CloudstateMapFieldValue, CloudstateObjectKey, CloudstateObjectValue,
        CloudstateRootKey, CloudstateRootValue,
    },
};

use redb::TableDefinition;

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
