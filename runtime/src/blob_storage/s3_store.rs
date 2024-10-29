use s3::Bucket;

use super::CloudstateBlobStorageEngine;

#[derive(Debug, Clone)]
pub struct S3BlobStore {
    bucket: Bucket,
}

impl S3BlobStore {
    pub fn new(bucket: Bucket) -> Self {
        Self { bucket }
    }
}

impl CloudstateBlobStorageEngine for S3BlobStore {
    fn get_blob_data(&self, blob_id: &str) -> Result<super::CloudstateBlobValue, anyhow::Error> {
        let res = self.bucket.get_object_blocking(blob_id)?;
        let binary = res.bytes().to_vec();
        Ok(binary.into())
    }

    fn put_blob(
        &self,
        blob_id: &str,
        blob_data: super::CloudstateBlobValue,
    ) -> Result<(), anyhow::Error> {
        let binary = blob_data.data;
        self.bucket.put_object_blocking(blob_id, &binary)?;

        Ok(())
    }

    fn delete_blob(&self, blob_id: &str) -> Result<(), anyhow::Error> {
        self.bucket.delete_object_blocking(blob_id)?;
        Ok(())
    }

    fn has_blob(&self, blob_id: &str) -> Result<bool, anyhow::Error> {
        match self.bucket.head_object_blocking(blob_id) {
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
