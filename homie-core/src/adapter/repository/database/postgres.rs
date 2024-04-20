use crate::adapter::repository::database::common::Session;
use crate::domain::dataset::DatasetPersist;

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

impl Session for Postgres {}

impl DatasetPersist for Postgres {
    fn read(&self, _key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling dataset read from Postgres.");
        Ok(true)
    }
}
