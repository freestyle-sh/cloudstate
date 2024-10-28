use std::{fs, path::PathBuf};

use super::CloudstateBlobStorage;

#[derive(Debug, Clone)]
pub struct FsBlobStore {
    root: PathBuf,
}

impl FsBlobStore {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl CloudstateBlobStorage for FsBlobStore {
    async fn get_blob(
        &mut self,
        blob_id: &str,
    ) -> Result<super::CloudstateBlobValue, anyhow::Error> {
        let binary = fs::read(self.root.join(blob_id))?;
        let blob_data = match bincode::deserialize(&binary) {
            Ok(blob_data) => blob_data,
            Err(e) => {
                tracing::error!("Failed to deserialize blob data: {:?}", e);
                return Err(e.into());
            }
        };
        Ok(blob_data)
    }

    async fn get_blob_size(&mut self, blob_id: &str) -> Result<usize, anyhow::Error> {
        Ok(fs::metadata(self.root.join(blob_id))?.len() as usize)
    }

    async fn put_blob(
        &mut self,
        blob_id: &str,
        blob_data: super::CloudstateBlobValue,
    ) -> Result<(), anyhow::Error> {
        let binary = bincode::serialize(&blob_data)?;
        fs::write(self.root.join(blob_id), binary)?;

        Ok(())
    }

    async fn delete_blob(&mut self, blob_id: &str) -> Result<(), anyhow::Error> {
        fs::remove_file(self.root.join(blob_id))?;
        Ok(())
    }

    async fn has_blob(&mut self, blob_id: &str) -> Result<bool, anyhow::Error> {
        fs::metadata(self.root.join(blob_id))
            .map(|_| true)
            .or_else(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(false)
                } else {
                    Err(e.into())
                }
            })
    }
}
