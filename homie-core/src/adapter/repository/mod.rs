use self::database::postgres::PostgresClient;
use crate::adapter::repository::database::http::HttpClient;
use crate::domain::hpi::HpiPersist;
use crate::domain::t_yield::TYieldPersist;
use crate::domain::zhvi::ZhviPersist;

pub mod database;

pub trait Persist: HpiPersist + TYieldPersist + ZhviPersist {}

pub struct Repository {
    client: Box<dyn Persist>,
}

impl Repository {
    // Define a new function taking Config as a parameter
    pub async fn new(config: &Config) -> Self {
        let client = Repository::establish_session(config).await;
        Repository { client }
    }

    // Method to establish session using the provided config
    async fn establish_session(config: &Config) -> Box<dyn Persist> {
        if config.use_db {
            println!("Using PostgreSQL client.");
            Box::new(PostgresClient::new(config).await)
        } else {
            println!("Using HTTP client.");
            Box::new(HttpClient)
        }
    }

    // Getter method to access the client
    pub fn session(&self) -> &dyn Persist {
        &*self.client
    }
}

// TODO: Refactor out
// Maybe create RepositoryConfig trait
pub struct Config {
    use_db: bool,
}

impl Config {
    pub fn load_config() -> Config {
        Config { use_db: true }
    }
}
