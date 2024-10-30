use std::sync::Arc;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::{extensions::cloudstate::Transaction, tables::BLOBS_TABLE};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CloudstateBlobValue {
    pub data: Vec<u8>,
    // pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CloudstateBlobMetadata {
    pub type_: String,
    // pub type_: String,
}

impl From<Vec<u8>> for CloudstateBlobValue {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

#[derive(Debug, Clone)]
pub struct CloudstateBlobStorage {
    inner_storage: Arc<dyn CloudstateBlobStorageEngine>,
}

impl CloudstateBlobStorage {
    pub fn new(inner_storage: Arc<dyn CloudstateBlobStorageEngine>) -> Self {
        Self { inner_storage }
    }

    pub fn get_blob_data(&self, blob_id: &str) -> Result<CloudstateBlobValue, Error> {
        self.inner_storage.get_blob_data(blob_id)
    }

    pub fn get_blob_size(&self, blob_id: &str) -> Result<usize, Error> {
        self.inner_storage.get_blob_size(blob_id)
    }

    pub fn put_blob(
        &self,
        blob_id: &str,
        transaction: &Transaction,
        blob_data: CloudstateBlobValue,
        blob_metadata: CloudstateBlobMetadata,
    ) -> Result<(), Error> {
        let mut blob_table = transaction.open_table(BLOBS_TABLE)?;
        blob_table.insert(&blob_id.into(), blob_metadata)?;
        self.inner_storage.put_blob(blob_id, blob_data)
    }

    pub fn delete_blob(&self, blob_id: &str, transaction: &Transaction) -> Result<(), Error> {
        let mut blob_table = transaction.open_table(BLOBS_TABLE)?;
        blob_table.remove(&blob_id.into())?;
        self.inner_storage.delete_blob(blob_id)
    }

    pub fn has_blob(&self, blob_id: &str) -> Result<bool, Error> {
        self.inner_storage.has_blob(blob_id)
    }

    pub fn get_blob_metadata(
        &self,
        blob_id: &str,
        transaction: &Transaction,
    ) -> Result<CloudstateBlobMetadata, Error> {
        let blob_table = transaction.open_table(BLOBS_TABLE)?;
        let out = match blob_table.get(&blob_id.into()) {
            Ok(Some(metadata)) => Ok(metadata.value()),
            Ok(None) => Err(Error::msg("Blob not found")),
            Err(e) => Err(e),
        };
        out
    }
}

impl Default for CloudstateBlobStorage {
    fn default() -> Self {
        Self::new(Arc::new(in_memory_store::InMemoryBlobStore::new()))
    }
}

pub trait CloudstateBlobStorageEngine: Send + Sync + std::fmt::Debug + 'static {
    fn get_blob_data(&self, blob_id: &str) -> Result<CloudstateBlobValue, Error>;
    fn get_blob_size(&self, blob_id: &str) -> Result<usize, Error> {
        Ok(self.get_blob_data(blob_id)?.data.len())
    }
    fn put_blob(&self, blob_id: &str, blob_data: CloudstateBlobValue) -> Result<(), Error>;
    fn delete_blob(&self, blob_id: &str) -> Result<(), Error>;
    fn has_blob(&self, blob_id: &str) -> Result<bool, Error>;
}

pub mod fs_store;
pub mod in_memory_store;
pub mod s3_store;
