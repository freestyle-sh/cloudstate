use std::{
    fs,
    io::{Read, Seek},
    path::PathBuf,
};

use super::CloudstateBlobStorageEngine;

#[derive(Debug, Clone)]
pub struct FsBlobStore {
    root: PathBuf,
}

impl FsBlobStore {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl CloudstateBlobStorageEngine for FsBlobStore {
    fn get_blob_data(&self, blob_id: &str) -> Result<super::CloudstateBlobValue, anyhow::Error> {
        let binary = fs::read(self.root.join(blob_id))?;
        Ok(binary.into())
    }

    fn get_blob_size(&self, blob_id: &str) -> Result<usize, anyhow::Error> {
        Ok(fs::metadata(self.root.join(blob_id))?.len() as usize)
    }

    fn put_blob(
        &self,
        blob_id: &str,
        blob_data: super::CloudstateBlobValue,
    ) -> Result<(), anyhow::Error> {
        // let binary = bincode::serialize(&blob_data)?;
        fs::write(self.root.join(blob_id), blob_data.data)?;

        Ok(())
    }

    fn delete_blob(&self, blob_id: &str) -> Result<(), anyhow::Error> {
        fs::remove_file(self.root.join(blob_id))?;
        Ok(())
    }

    fn has_blob(&self, blob_id: &str) -> Result<bool, anyhow::Error> {
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

    fn get_blob_slice(
        &self,
        blob_id: &str,
        start: Option<i32>,
        end: Option<i32>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let mut file = fs::File::open(self.root.join(blob_id))?;
        let start = match start {
            Some(s) => s as usize,
            None => 0,
        };
        let end = match end {
            Some(e) => e as usize,
            None => fs::metadata(self.root.join(blob_id))?.len() as usize,
        };
        file.seek(std::io::SeekFrom::Start(start as u64))?;
        let mut buf = vec![0; end - start];
        file.read_exact(&mut buf)?;
        Ok(buf)
    }
}
