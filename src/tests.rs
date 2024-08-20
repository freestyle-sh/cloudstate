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
fn test() {
    // let mut fields = HashMap::new();
    // fields.insert("test".to_string(), CloudstatePrimitiveData::Number(1.0));
    // let string =
    //     deno_core::serde_json::to_string(&CloudstateObjectData { fields: fields }).unwrap();

    // println!("{:?}", string);

    let _ = run_script(
        "tests/test_object.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    );
}
