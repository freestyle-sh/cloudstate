# Runtime Testing

## Overview

Our runtime tests help us ensure stability and correctness of the Cloudstate runtime. We prefer to have many small tests rather than a few large tests, as this makes it easier to identify the cause of a failure.

## Writing Tests

All tests go in the `runtime/tests` directory. Each test should be in its own file, and should be named based on what it tests.

For a test to be run, please add it to `src/tests.rs` in the `runtime` directory. Please add it in the correct alphabetical order.
