use redb::ReadableTable;
use tracing::debug;

use crate::tables::{ARRAYS_TABLE, BLOBS_TABLE, OBJECTS_TABLE, ROOTS_TABLE};

pub fn print_database(db: &redb::Database) {
    let txn = db.begin_read().unwrap();

    // debug!("Objects Table");
    // if let Ok(table) = txn.open_table(OBJECTS_TABLE) {
    //     for entry in table.iter().unwrap() {
    //         let entry = entry.unwrap();
    //         debug!("{:#?}: {:#?}", entry.0.value().id, entry.1.value().data);
    //     }
    // }

    // debug!("Arrays Table");
    // if let Ok(table) = txn.open_table(ARRAYS_TABLE) {
    //     for entry in table.iter().unwrap() {
    //         let entry = entry.unwrap();
    //         debug!("{:#?}: {:#?}", entry.0.value().id, entry.1.value().data);
    //     }
    // }

    debug!("Roots Table");
    if let Ok(table) = txn.open_table(ROOTS_TABLE) {
        for entry in table.iter().unwrap() {
            let entry = entry.unwrap();
            debug!("{:#?}: {:#?}", entry.0.value().alias, entry.1.value().id);
        }
    }

    // debug!("Blobs Table");
    // if let Ok(table) = txn.open_table(BLOBS_TABLE) {
    //     for entry in table.iter().unwrap() {
    //         let entry = entry.unwrap();
    //         debug!(
    //             "{:#?}: ({:#?}) {:#?}",
    //             entry.0.value().id,
    //             entry.1.value().data.len(),
    //             entry.1.value().type_,
    //         );
    //     }
    // }
}
