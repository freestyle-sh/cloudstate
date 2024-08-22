use crate::{
    execution::run_script,
    extensions::cloudstate::{CloudstateObjectData, CloudstatePrimitiveData, ReDBCloudstate},
    gc::mark_and_sweep,
    tables,
};
use redb::{backends::InMemoryBackend, Database, ReadableTable};
use std::collections::HashMap;

#[test]
fn test_object() {
    let _ = run_script(
        "tests/objects_and_arrays.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    )
    .unwrap();
}

#[test]
fn test_maps() {
    let _ = run_script(
        "tests/maps.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    )
    .unwrap();
}

#[test]
fn test_simple_objects() {
    let _ = run_script(
        "tests/simple_objects.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    )
    .unwrap();
}

#[test]
fn test_date() {
    let _ = run_script(
        "tests/dates.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    )
    .unwrap();
}

#[test]
fn test_bigint() {
    let _ = run_script(
        "tests/bigints.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    )
    .unwrap();
}

#[test]
fn test_nested_objects() {
    let _ = run_script(
        "tests/nested_objects.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    )
    .unwrap();
}

#[test]
fn gc_test() {
    let mut db = Database::builder()
        .create_with_backend(InMemoryBackend::default())
        .unwrap();
    let mut cloudstate = ReDBCloudstate {
        db: db,
        transactions: HashMap::new(),
    };

    let (cloudstate, _) = run_script("tests/gc_test.js", cloudstate).unwrap();

    let mut db = cloudstate.db;
    let read = db.begin_read();
    let read = match read {
        Ok(read) => read,
        Err(e) => panic!("Error reading database: {}", e),
    };
    {
        let objects_table = match read.open_table(tables::OBJECTS_TABLE) {
            Ok(table) => table,
            Err(e) => panic!("Error opening objects table: {}", e),
        };
        let mut count = 0;
        for item in objects_table.iter().unwrap() {
            if let Ok((key, value)) = item {
                count += 1;
            }
        }
        assert_eq!(count, 5);
    }
    read.close().unwrap();

    mark_and_sweep(&db).unwrap();
    let read = db.begin_read().unwrap();

    {
        let objects_table = match read.open_table(tables::OBJECTS_TABLE) {
            Ok(table) => table,
            Err(e) => panic!("Error opening objects table: {}", e),
        };
        let mut count = 0;
        println!("\n\n");
        for item in objects_table.iter().unwrap() {
            if let Ok((key, value)) = item {
                println!("DATA: {:?}", value.value().data);
                count += 1;
            }
        }
        assert_eq!(count, 3);
    }
    read.close().unwrap();
}
