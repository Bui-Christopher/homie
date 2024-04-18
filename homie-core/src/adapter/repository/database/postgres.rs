use std::error::Error;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::CRUDOperations;

#[derive(Debug, Serialize, Deserialize)]
pub struct Postgres {}

impl Postgres {
    pub fn new() -> Self {
        Postgres {}
    }
}

impl Default for Postgres {
    fn default() -> Self {
        Postgres::new()
    }
}

impl<T: Serialize + DeserializeOwned + Debug> CRUDOperations<T> for Postgres {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Postgres: Create object: {:?}", obj);
        Ok(true)
    }

    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("Postgres: Read object by key: {:?}", key);
        Ok(true)
    }

    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>> {
        println!("Postgres: Update object: {:?}", obj);
        Ok(true)
    }

    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        println!("Postgres: Delete object by key: {:?}", key);
        Ok(true)
    }
}
