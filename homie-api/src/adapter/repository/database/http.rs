use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::adapter::repository::CRUDOperations;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }
}

impl<T: Serialize + Deserialize<'static> + std::fmt::Debug> CRUDOperations<T> for HttpClient {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("HttpClient: Create object: {:?}", obj);
        Ok(true)
    }

    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("HttpClient: Read object by key: {:?}", key);
        Ok(true)
    }

    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("HttpClient: Update object: {:?}", obj);
        Ok(true)
    }

    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("HttpClient: Delete object by key: {:?}", key);
        Ok(true)
    }
}
