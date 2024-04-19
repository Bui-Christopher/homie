use std::error::Error;
use std::fmt::Debug;

pub trait CRUDOperations<T>: Debug + Send + Sync {
    fn create(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn read(&self, key: &str) -> Result<bool, Box<dyn Error>>;
    fn update(&self, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, key: &str) -> Result<bool, Box<dyn Error>>;
}
