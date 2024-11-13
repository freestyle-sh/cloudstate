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
        let res = self.bucket.get_object(blob_id)?;
        let binary = res.bytes().to_vec();
        Ok(binary.into())
    }

    fn put_blob(
        &self,
        blob_id: &str,
        blob_data: super::CloudstateBlobValue,
    ) -> Result<(), anyhow::Error> {
        let binary = blob_data.data;
        self.bucket.put_object(blob_id, &binary)?;

        Ok(())
    }

    fn delete_blob(&self, blob_id: &str) -> Result<(), anyhow::Error> {
        self.bucket.delete_object(blob_id)?;
        Ok(())
    }

    fn get_blob_slice(
        &self,
        blob_id: &str,
        start: Option<i32>,
        end: Option<i32>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let start = match start {
            Some(s) => s as u64,
            None => 0,
        };
        let end = match end {
            Some(e) => Some(e as u64),
            None => None,
        };

        let res = self.bucket.get_object_range(blob_id, start, end)?;

        Ok(res.bytes().to_vec())
    }

    fn has_blob(&self, blob_id: &str) -> Result<bool, anyhow::Error> {
        match self.bucket.head_object(blob_id) {
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
