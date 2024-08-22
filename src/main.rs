pub mod bincode;
pub mod execution;
pub mod extensions;
pub mod gc;
pub mod tables;
#[cfg(test)]
mod tests;

use execution::run_script;
use extensions::cloudstate::ReDBCloudstate;
use redb::{backends::InMemoryBackend, Database};
use std::collections::HashMap;

fn main() {
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
