use cloudstate_runtime::{
    blob_storage::CloudstateBlobStorage, extensions::cloudstate::ReDBCloudstate,
};

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
    ) -> impl std::future::Future<Output = String> + Send;
}
