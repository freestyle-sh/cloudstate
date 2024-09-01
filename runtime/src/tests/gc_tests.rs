use std::collections::HashMap;

use redb::{backends::InMemoryBackend, Database, ReadableTable};

use crate::{
    execution::run_script, extensions::cloudstate::ReDBCloudstate, gc::mark_and_sweep, tables,
};

#[test]
fn test_gc_objects() {
    let db = Database::builder()
        .create_with_backend(InMemoryBackend::default())
        .unwrap();
    let cloudstate = ReDBCloudstate {
        db,
        transactions: HashMap::new(),
    };

    let (cloudstate, _) = run_script("tests/gc/base.js", cloudstate).unwrap();

    let db = &cloudstate.lock().unwrap().db;
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
            if let Ok((_key, _value)) = item {
                count += 1;
            }
        }
        assert_eq!(count, 5);
    }

    read.close().unwrap();

    // Run the garbage collector
    mark_and_sweep(&db).unwrap();

    let read = db.begin_read().unwrap();

    {
        let objects_table = match read.open_table(tables::OBJECTS_TABLE) {
            Ok(table) => table,
            Err(e) => panic!("Error opening objects table: {}", e),
        };
        let mut count = 0;
        for item in objects_table.iter().unwrap() {
            if let Ok((_key, _value)) = item {
                count += 1;
            }
        }
        assert_eq!(count, 3);
    }
    read.close().unwrap();
}

//TODO: THIS PROCESS SHOULD BE FUNCTION-IZED AND REUSED CUZ IT'S THE SAME AS THE ONE ABOVE

#[test]
fn test_gc_maps() {
    let db = Database::builder()
        .create_with_backend(InMemoryBackend::default())
        .unwrap();
    let cloudstate = ReDBCloudstate {
        db: db,
        transactions: HashMap::new(),
    };

    let (cloudstate, _) = run_script("tests/gc/map.js", cloudstate).unwrap();

    let db = &cloudstate.lock().unwrap().db;
    let read = db.begin_read();
    let read = match read {
        Ok(read) => read,
        Err(e) => panic!("Error reading database: {}", e),
    };
    {
        let map_table = match read.open_table(tables::MAPS_TABLE) {
            Ok(table) => table,
            Err(e) => panic!("Error opening objects table: {}", e),
        };
        let mut count = 0;
        for item in map_table.iter().unwrap() {
            if let Ok((_key, _value)) = item {
                count += 1;
            }
        }
        assert_eq!(count, 2);
    }
    read.close().unwrap();

    // Run the garbage collector
    mark_and_sweep(&db).unwrap();

    let read = db.begin_read();
    let read = match read {
        Ok(read) => read,
        Err(e) => panic!("Error reading database: {}", e),
    };
    {
        let map_table = match read.open_table(tables::MAPS_TABLE) {
            Ok(table) => table,
            Err(e) => panic!("Error opening objects table: {}", e),
        };
        let mut count = 0;
        for item in map_table.iter().unwrap() {
            if let Ok((_key, _value)) = item {
                count += 1;
            }
        }
        assert_eq!(count, 0);
    }
}

#[test]
pub fn test_gc_array() {
    let db = Database::builder()
        .create_with_backend(InMemoryBackend::default())
        .unwrap();
    let cloudstate = ReDBCloudstate {
        db,
        transactions: HashMap::new(),
    };

    let (cloudstate, _) = run_script("tests/gc/array.js", cloudstate).unwrap();

    let db = &cloudstate.lock().unwrap().db;
    let read = db.begin_read();
    let read = match read {
        Ok(read) => read,
        Err(e) => panic!("Error reading database: {}", e),
    };
    {
        let array_table = match read.open_table(tables::ARRAYS_TABLE) {
            Ok(table) => table,
            Err(e) => panic!("Error opening objects table: {}", e),
        };
        let mut count = 0;
        for item in array_table.iter().unwrap() {
            if let Ok((_key, _value)) = item {
                count += 1;
            }
        }
        assert_eq!(count, 9);
    }

    read.close().unwrap();

    // Run the garbage collector
    mark_and_sweep(&db).unwrap();

    let read = db.begin_read();
    let read = match read {
        Ok(read) => read,
        Err(_e) => return,
    };
    {
        let array_table = match read.open_table(tables::ARRAYS_TABLE) {
            Ok(table) => table,
            Err(e) => panic!("Error opening objects table: {}", e),
        };
        let mut count = 0;
        for item in array_table.iter().unwrap() {
            if let Ok((_key, _value)) = item {
                count += 1;
            }
        }
        assert_eq!(count, 5);
    }
}
