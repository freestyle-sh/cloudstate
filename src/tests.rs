use crate::execution::run_script;

#[test]
fn test_object() {
    run_script("tests/objects_and_arrays.js").unwrap();
}

#[test]
fn test_maps() {
    run_script("tests/maps.js").unwrap();
}
