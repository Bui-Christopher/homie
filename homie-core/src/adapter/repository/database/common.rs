use std::error::Error;

use crate::domain::dataset::DatasetPersist;

pub trait FilePersist<S, Q> {
    fn create(&self, session: &S) -> Result<bool, Box<dyn Error>>;
    fn read(&self, session: &S, key: &str) -> Result<bool, Box<dyn Error>>;
    fn read_by_query(&self, session: &S, query: Q) -> Result<bool, Box<dyn Error>>;
    fn update(&self, session: &S) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, session: &S, key: &str) -> Result<bool, Box<dyn Error>>;
}

pub trait PostgresPersist<S, Q> {
    fn create(&self, session: &S) -> Result<bool, Box<dyn Error>>;
    fn read(&self, session: &S, key: &str) -> Result<bool, Box<dyn Error>>;
    fn read_by_query(&self, session: &S, query: Q) -> Result<bool, Box<dyn Error>>;
    fn update(&self, session: &S) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, session: &S, key: &str) -> Result<bool, Box<dyn Error>>;
}

pub trait HttpPersist<S, Q> {
    fn read(&self, session: &S, id: &str) -> Result<bool, Box<dyn Error>>;
    fn read_by_query(&self, session: &S, query: Q) -> Result<bool, Box<dyn Error>>;
}

pub trait Session: Send + Sync + DatasetPersist {}
