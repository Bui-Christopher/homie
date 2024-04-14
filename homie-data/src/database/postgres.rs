use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::database::common::CRUDOperations;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Postgres {}

// impl Postgres {
//     pub fn new() -> Self {
//         Postgres {}
//     }
// }

impl<T: Serialize + Deserialize<'static> + std::fmt::Debug> CRUDOperations<T> for Postgres {
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
