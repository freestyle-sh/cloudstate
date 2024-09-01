use redb::ReadableTable;
use tracing::event;

use crate::tables::{ARRAYS_TABLE, OBJECTS_TABLE, ROOTS_TABLE};

pub fn print_database(db: &redb::Database) {
    event!(tracing::Level::DEBUG, "Objects Table");
    let txn = db.begin_read().unwrap();
    if let Ok(table) = txn.open_table(OBJECTS_TABLE) {
        for entry in table.iter().unwrap() {
            let entry = entry.unwrap();
            event!(
                tracing::Level::DEBUG,
                "{:#?}: {:#?}",
                entry.0.value().id,
                entry.1.value().data
            );
        }
    }

    event!(tracing::Level::DEBUG, "Arrays Table");
    if let Ok(table) = txn.open_table(ARRAYS_TABLE) {
        for entry in table.iter().unwrap() {
            let entry = entry.unwrap();
            event!(
                tracing::Level::DEBUG,
                "{:#?}: {:#?}",
                entry.0.value().id,
                entry.1.value().data
            );
        }
    }

    event!(tracing::Level::DEBUG, "Roots Table");
    if let Ok(table) = txn.open_table(ROOTS_TABLE) {
        for entry in table.iter().unwrap() {
            let entry = entry.unwrap();
            event!(
                tracing::Level::DEBUG,
                "{:#?}: {:#?}",
                entry.0.value().alias,
                entry.1.value().id
            );
        }
    }
}
