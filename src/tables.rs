use redb::TableDefinition;
use crate::{bincode::Bincode, extensions::cloudstate::{CloudstateMapFieldKey, CloudstateMapFieldValue, CloudstateObjectKey, CloudstateObjectValue, CloudstateRootKey, CloudstateRootValue}};


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