mod bincode;
mod execution;
mod extensions;
mod print;
mod tables;
mod v8_macros;

fn main() {
    tracing_subscriber::fmt::init();

    let (cs, result) = crate::execution::run_script(
        &format!("tests/{}.js", "array_push_object"),
        crate::extensions::cloudstate::ReDBCloudstate::new(std::sync::Arc::new(
            std::sync::Mutex::new(
                redb::Database::builder()
                    .create_with_backend(redb::backends::InMemoryBackend::default())
                    .unwrap(),
            ),
        )),
    )
    .unwrap();
    crate::print::print_database(&cs.get_database_mut());
    result.unwrap();
}
