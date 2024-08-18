pub mod execution;
pub mod extensions;

use deno_core::anyhow::Error;

fn main() -> Result<(), Error> {
    execution::run_script("examples/main.js")
}
