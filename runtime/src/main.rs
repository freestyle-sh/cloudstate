use blob_storage::in_memory_store::InMemoryBlobStore;

mod bincode;
mod blob_storage;
mod execution;
mod extensions;
mod print;
mod tables;
mod v8_macros;
fn main() {
    tracing_subscriber::fmt::init();

    let (cs, result) = crate::execution::run_script(
        &format!("tests/{}.js", "array_on_object_reference"),
        crate::extensions::cloudstate::ReDBCloudstate::new(std::sync::Arc::new(
            std::sync::Mutex::new(
                redb::Database::builder()
                    .create_with_backend(redb::backends::InMemoryBackend::default())
                    .unwrap(),
            ),
        )),
        InMemoryBlobStore::new(),
    )
    .unwrap();
    crate::print::print_database(&cs.get_database_mut());
    result.unwrap();
}
