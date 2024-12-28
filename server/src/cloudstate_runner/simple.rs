use crate::CloudstateRunner;

use super::execute::execute_script;

#[derive(Clone)]
pub struct SimpleCloudstateRunner {}

impl SimpleCloudstateRunner {
    pub fn new() -> Self {
        Self {}
    }
}

impl CloudstateRunner for SimpleCloudstateRunner {
    async fn run_cloudstate(
        &self,
        script: &str,
        classes_script: &str,
        cs: cloudstate_runtime::extensions::cloudstate::ReDBCloudstate,
        blob_storage: cloudstate_runtime::blob_storage::CloudstateBlobStorage,
    ) -> String {
        execute_script(script, classes_script, cs, blob_storage).await
    }
}
