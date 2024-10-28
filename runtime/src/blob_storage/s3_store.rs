use s3::Bucket;

use super::CloudstateBlobStorage;

#[derive(Debug, Clone)]
pub struct S3BlobStore {
    bucket: Bucket,
}

impl S3BlobStore {
    pub fn new(bucket: Bucket) -> Self {
        Self { bucket }
    }
}

impl CloudstateBlobStorage for S3BlobStore {
    async fn get_blob(&self, blob_id: &str) -> Result<super::CloudstateBlobValue, anyhow::Error> {
        let res = self.bucket.get_object(blob_id).await?;
        let binary = res.bytes();
        let blob_data = match bincode::deserialize(&binary) {
            Ok(blob_data) => blob_data,
            Err(e) => {
                tracing::error!("Failed to deserialize blob data: {:?}", e);
                return Err(e.into());
            }
        };
        Ok(blob_data)
    }

    fn put_blob(
        &mut self,
        blob_id: &str,
        blob_data: super::CloudstateBlobValue,
    ) -> Result<(), anyhow::Error> {
        let binary = bincode::serialize(&blob_data)?;
        self.bucket.put_object(blob_id, &binary);

        Ok(())
    }

    fn delete_blob(&mut self, blob_id: &str) -> Result<(), anyhow::Error> {
        self.bucket.delete_object(blob_id).await?;
        Ok(())
    }

    async fn has_blob(&mut self, blob_id: &str) -> Result<bool, anyhow::Error> {
        match self.bucket.head_object(blob_id).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("404") {
                    Ok(false)
                } else {
                    Err(e.into())
                }
            }
        }
    }
}
