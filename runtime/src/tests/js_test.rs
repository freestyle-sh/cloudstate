#[macro_export]
macro_rules! js_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let _ = tracing_subscriber::fmt::try_init();

            let (cs, result) = $crate::execution::run_script(
                &format!("tests/{}.js", stringify!($name)),
                $crate::extensions::cloudstate::ReDBCloudstate::new(std::sync::Arc::new(
                    std::sync::Mutex::new(
                        redb::Database::builder()
                            .create_with_backend(redb::backends::InMemoryBackend::default())
                            .unwrap(),
                    ),
                )),
                $crate::blob_storage::CloudstateBlobStorage::new(std::sync::Arc::new(
                    $crate::blob_storage::in_memory_store::InMemoryBlobStore::default(),
                )),
            )
            .unwrap();
            $crate::print::print_database(&cs.get_database_mut());
            result.unwrap();
        }
    };
}
