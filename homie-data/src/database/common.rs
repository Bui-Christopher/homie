use std::error::Error;

use serde::{Deserialize, Serialize};

// TODO: Refactor Deserialize<'static> to DeserializeOwned
// TODO: Rename all std::fmt::Debug to just Debug
pub trait CRUDOperations<T>: Serialize + Deserialize<'static> + std::fmt::Debug {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>>;
    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>>;
}
