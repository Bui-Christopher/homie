use crate::adapter::repository::Persist;
use crate::domain::t_yield::TYieldPersist;

pub struct PostgresClient;

impl PostgresClient {
    pub fn new() -> Self {
        PostgresClient {}
    }
}

impl Default for PostgresClient {
    fn default() -> Self {
        PostgresClient::new()
    }
}

impl TYieldPersist for PostgresClient {
    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling t_yield read with id: {id}  from Postgres.");
        Ok(true)
    }
}

impl Persist for PostgresClient {}
