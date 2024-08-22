use crate::{
    execution::run_script,
    extensions::cloudstate::{CloudstateObjectData, CloudstatePrimitiveData, ReDBCloudstate},
};
use redb::{backends::InMemoryBackend, Database};
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
