use crate::adapter::repository::database::common::Session;
use crate::domain::dataset::DatasetPersist;

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

impl Session for FileStorage {}

impl DatasetPersist for FileStorage {
    fn read(&self, _key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling dataset read from FileStorage.");
        Ok(true)
    }
}
