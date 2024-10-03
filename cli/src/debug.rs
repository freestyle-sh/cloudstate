use cloudstate_runtime::{
    print::print_database,
    tables::{ARRAYS_TABLE, MAPS_TABLE, OBJECTS_TABLE, ROOTS_TABLE},
};
use redb::{Database, ReadableTable};

#[test]
fn test_print() {
    tracing_subscriber::fmt::init();
    print_database(
        &Database::open("/Users/jacobzwang/Documents/GitHub/xcloudplatform/cloudstate").unwrap(),
    );
}

#[test]
fn test_length() {
    let db =
        &Database::open("/Users/jacobzwang/Documents/GitHub/xcloudplatform/cloudstate").unwrap();
    let transaction = db.begin_read().unwrap();
    let count = transaction
        .open_table(OBJECTS_TABLE)
        .unwrap()
        .iter()
        .unwrap()
        .count();
    println!("{count}");
}
