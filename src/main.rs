pub mod execution;
pub mod extensions;

#[cfg(test)]
mod tests;

use deno_core::anyhow::Error;
use extensions::cloudstate::{cloudstate, ReDBCloudstate};
use redb::{backends::InMemoryBackend, Database};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let cloudstate = ReDBCloudstate {
        db: Database::builder()
            .create_with_backend(InMemoryBackend::default())
            .unwrap(),
        transactions: HashMap::new(),
    };

    let (cloudstate, _) = execution::run_script("examples/main.js", cloudstate).unwrap();
    match execution::run_script("examples/main.js", cloudstate) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
