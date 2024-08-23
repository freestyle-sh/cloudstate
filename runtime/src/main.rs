pub mod bincode;
pub mod execution;
pub mod extensions;
pub mod gc;
mod print;
pub mod tables;
#[cfg(test)]
mod tests;

use execution::run_script;
use extensions::cloudstate::ReDBCloudstate;
use print::print_database;
use redb::{backends::InMemoryBackend, Database};
use std::collections::HashMap;

fn main() {
    let cs = ReDBCloudstate {
        db: Database::builder()
            .create_with_backend(InMemoryBackend::default())
            .unwrap(),
        transactions: HashMap::new(),
    };

    let (cs, result) = run_script("tests/objects_and_arrays.js", cs).unwrap();

    print_database(&cs.db);

    result.unwrap();
}
