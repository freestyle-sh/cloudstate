use std::collections::HashMap;

use cloudstate_runtime::{
    execution::run_script, extensions::cloudstate::ReDBCloudstate, print::print_database,
};
use redb::{backends::InMemoryBackend, Database};
mod bincode;
mod execution;
mod extensions;
mod gc;
mod print;
mod tables;

fn main() {
    let (cs, result) = run_script(
        "tests/fetch.js",
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
    )
    .unwrap();

    print_database(&cs.db);
    result.unwrap();
}
