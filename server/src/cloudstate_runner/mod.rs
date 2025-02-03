use cloudstate_runtime::{
    blob_storage::CloudstateBlobStorage, extensions::cloudstate::ReDBCloudstate,
};

use crate::ServerInfo;

pub mod execute;
pub mod module_loader;
pub mod simple;
pub trait CloudstateRunner: Send + Sync + Clone {
    fn run_cloudstate(
        &self,
        script: &str,
        classes_script: &str,
        cs: ReDBCloudstate,
        blob_storage: CloudstateBlobStorage,
        request_info: ServerInfo,
    ) -> impl std::future::Future<Output = String> + Send;
}
