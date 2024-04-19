use std::error::Error;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::adapter::repository::CRUDOperations;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStorage {}

impl FileStorage {
    pub fn new() -> Self {
        FileStorage {}
    }
}

impl Default for FileStorage {
    fn default() -> Self {
        FileStorage::new()
    }
}

impl<T: Debug> CRUDOperations<T> for FileStorage {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("File: Create object: {:?}", obj);
        Ok(true)
    }

    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("File: Read object by key: {:?}", key);
        Ok(true)
    }

    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("File: Update object: {:?}", obj);
        Ok(true)
    }

    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("File: Delete object by key: {:?}", key);
        Ok(true)
    }
}
