use self::database::postgres::PostgresClient;
use crate::adapter::config::Config;
use crate::adapter::repository::database::http::HttpClient;
use crate::domain::hpi::HpiPersist;
use crate::domain::t_yield::TYieldPersist;
use crate::domain::zhvi::ZhviPersist;
use crate::error::Error;

pub mod database;

pub trait Persist: HpiPersist + TYieldPersist + ZhviPersist {}

pub struct Repository {
    client: Box<dyn Persist>,
}

impl Repository {
    // Define a new function taking Config as a parameter
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let client = Repository::establish_session(config).await?;
        Ok(Repository { client })
    }

    // Method to establish session using the provided config
    async fn establish_session(config: &Config) -> Result<Box<dyn Persist>, Error> {
        if config.is_db_enabled() {
            println!("Using PostgreSQL client.");
            Ok(Box::new(PostgresClient::new(config).await?))
        } else {
            println!("Using HTTP client.");
            Ok(Box::new(HttpClient))
        }
    }

    // Getter method to access the client
    pub fn session(&self) -> &dyn Persist {
        &*self.client
    }
}
