#[macro_export]
macro_rules! js_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let (cs, result) = crate::execution::run_script(
                &format!("tests/{}.js", stringify!($name)),
                crate::extensions::cloudstate::ReDBCloudstate {
                    db: redb::Database::builder()
                        .create_with_backend(redb::backends::InMemoryBackend::default())
                        .unwrap(),
                    transactions: std::collections::HashMap::new(),
                },
            )
            .unwrap();
            crate::print::print_database(&cs.lock().unwrap().db);
            result.unwrap();
        }
    };
}
