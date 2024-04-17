use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::adapter::repository::CRUDOperations;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FileStorage {}

impl FileStorage {
    pub(crate) fn new() -> Self {
        FileStorage {}
    }
}

impl<T: Serialize + Deserialize<'static> + std::fmt::Debug> CRUDOperations<T> for FileStorage {
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
