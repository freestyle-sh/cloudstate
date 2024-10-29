use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CloudstateBlobValue {
    pub data: Vec<u8>,
    pub type_: String,
}

pub trait CloudstateBlobStorage: Send + Sync {
    fn get_blob(&self, blob_id: &str) -> Result<CloudstateBlobValue, Error>;
    fn get_blob_size(&self, blob_id: &str) -> Result<usize, Error> {
        Ok(self.get_blob(blob_id)?.data.len())
    }
    fn put_blob(&self, blob_id: &str, blob_data: CloudstateBlobValue) -> Result<(), Error>;
    fn delete_blob(&self, blob_id: &str) -> Result<(), Error>;
    fn has_blob(&self, blob_id: &str) -> Result<bool, Error>;
}

pub mod fs_store;
pub mod in_memory_store;
pub mod s3_store;
