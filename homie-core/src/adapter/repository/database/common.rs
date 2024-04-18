use std::error::Error;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait CRUDOperations<T>: Serialize + DeserializeOwned + Debug {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>>;
    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>>;
}
