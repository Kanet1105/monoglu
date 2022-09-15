use crate::error::Exception;
use serde::Deserialize;
use sled;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub(crate) struct StorageBuilder {
    path: Vec<String>,
    capacity: u64,
}

impl StorageBuilder {
    pub fn new(self) -> Result<sled::Db, Exception> {
        let path: PathBuf = self.path
            .iter()
            .collect();
        let storage = sled::open(path)?;
        Ok(storage)
    }
}