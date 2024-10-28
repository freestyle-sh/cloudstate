use super::CloudstateBlobStorage;

#[derive(Debug, Clone)]
pub struct InMemoryBlobStore {
    blobs: std::collections::HashMap<String, super::CloudstateBlobValue>,
}

impl InMemoryBlobStore {
    pub fn new() -> Self {
        Self {
            blobs: std::collections::HashMap::new(),
        }
    }
}

impl CloudstateBlobStorage for InMemoryBlobStore {
    async fn get_blob(
        &mut self,
        blob_id: &str,
    ) -> Result<super::CloudstateBlobValue, anyhow::Error> {
        match self.blobs.get(blob_id) {
            Some(blob_data) => Ok(blob_data.clone()),
            None => {
                tracing::error!("Blob not found: {}", blob_id);
                Err(anyhow::anyhow!("Blob not found: {}", blob_id))
            }
        }
    }

    async fn get_blob_size(&mut self, blob_id: &str) -> Result<usize, anyhow::Error> {
        match self.blobs.get(blob_id) {
            Some(blob_data) => Ok(blob_data.data.len()),
            None => {
                tracing::error!("Blob not found: {}", blob_id);
                Err(anyhow::anyhow!("Blob not found: {}", blob_id))
            }
        }
    }

    async fn put_blob(
        &mut self,
        blob_id: &str,
        blob_data: super::CloudstateBlobValue,
    ) -> Result<(), anyhow::Error> {
        self.blobs.insert(blob_id.to_string(), blob_data);
        Ok(())
    }

    async fn delete_blob(&mut self, blob_id: &str) -> Result<(), anyhow::Error> {
        self.blobs.remove(blob_id);
        Ok(())
    }

    async fn has_blob(&mut self, blob_id: &str) -> Result<bool, anyhow::Error> {
        Ok(self.blobs.contains_key(blob_id))
    }
}
