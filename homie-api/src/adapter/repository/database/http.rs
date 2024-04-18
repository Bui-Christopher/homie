use std::error::Error;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::CRUDOperations;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }
}

impl<T: Serialize + DeserializeOwned + Debug> CRUDOperations<T> for HttpClient {
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
