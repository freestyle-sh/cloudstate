use super::CloudstateBlobStorageEngine;

#[derive(Debug)]
pub struct InMemoryBlobStore {
    blobs: std::sync::RwLock<std::collections::HashMap<String, super::CloudstateBlobValue>>,
}

impl InMemoryBlobStore {
    pub fn new() -> Self {
        Self {
            blobs: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

impl Default for InMemoryBlobStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudstateBlobStorageEngine for InMemoryBlobStore {
    fn get_blob_data(&self, blob_id: &str) -> Result<super::CloudstateBlobValue, anyhow::Error> {
        let blobs = self.blobs.read().unwrap();
        match blobs.get(blob_id) {
            Some(blob_data) => Ok(blob_data.clone()),
            None => {
                tracing::error!("Blob not found: {}", blob_id);
                Err(anyhow::anyhow!("Blob not found: {}", blob_id))
            }
        }
    }

    fn get_blob_size(&self, blob_id: &str) -> Result<usize, anyhow::Error> {
        let blobs = self.blobs.read().unwrap();
        match blobs.get(blob_id) {
            Some(blob_data) => Ok(blob_data.data.len()),
            None => {
                tracing::error!("Blob not found: {}", blob_id);
                Err(anyhow::anyhow!("Blob not found: {}", blob_id))
            }
        }
    }

    fn put_blob(
        &self,
        blob_id: &str,
        blob_data: super::CloudstateBlobValue,
    ) -> Result<(), anyhow::Error> {
        let mut blobs = self.blobs.write().unwrap();
        blobs.insert(blob_id.to_string(), blob_data);
        Ok(())
    }

    fn delete_blob(&self, blob_id: &str) -> Result<(), anyhow::Error> {
        let mut blobs = self.blobs.write().unwrap();
        blobs.remove(blob_id);

        Ok(())
    }

    fn has_blob(&self, blob_id: &str) -> Result<bool, anyhow::Error> {
        let blobs = self.blobs.read().unwrap();
        Ok(blobs.contains_key(blob_id))
    }

    fn get_blob_slice(
        &self,
        blob_id: &str,
        start: Option<i32>,
        end: Option<i32>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let blobs = self.blobs.read().unwrap();
        match blobs.get(blob_id) {
            Some(blob_data) => {
                let data = &blob_data.data;
                let start = start.unwrap_or(0) as usize;
                let end = end.unwrap_or(data.len() as i32) as usize;
                Ok(data[start..end].to_vec())
            }
            None => {
                tracing::error!("Blob not found: {}", blob_id);
                Err(anyhow::anyhow!("Blob not found: {}", blob_id))
            }
        }
    }
}
