use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::database::common::CRUDOperations;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObject {}

pub trait Persistence<T, D: CRUDOperations<T>> {
    fn create(&self, db: &D) -> Result<bool, Box<dyn Error>>;
    fn read(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>>;
    fn update(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>>;
}

impl<D: CRUDOperations<MyObject>> Persistence<MyObject, D> for MyObject {
    fn create(&self, db: &D) -> Result<bool, Box<dyn Error>> {
        db.create(self)
    }

    fn read(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>> {
        db.read(key)
    }

    fn update(&self, db: &D, obj: &MyObject) -> Result<bool, Box<dyn Error>> {
        db.update(obj)
    }

    fn delete(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>> {
        db.delete(key)
    }
}
