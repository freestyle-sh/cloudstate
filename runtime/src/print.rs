use redb::ReadableTable;

use crate::{
    print,
    tables::{ARRAYS_TABLE, OBJECTS_TABLE, ROOTS_TABLE},
};

pub fn print_database(db: &redb::Database) {
    println!("Objects Table");
    let txn = db.begin_read().unwrap();
    if let Ok(table) = txn.open_table(OBJECTS_TABLE) {
        for entry in table.iter().unwrap() {
            let entry = entry.unwrap();
            println!("{:#?}: {:#?}", entry.0.value().id, entry.1.value().data);
        }
    }

    println!("Arrays Table");
    if let Ok(table) = txn.open_table(ARRAYS_TABLE) {
        for entry in table.iter().unwrap() {
            let entry = entry.unwrap();
            println!("{:#?}: {:#?}", entry.0.value().id, entry.1.value().data);
        }
    }

    println!("Roots Table");
    if let Ok(table) = txn.open_table(ROOTS_TABLE) {
        for entry in table.iter().unwrap() {
            let entry = entry.unwrap();
            println!("{:#?}: {:#?}", entry.0.value().alias, entry.1.value().id);
        }
    }
}
