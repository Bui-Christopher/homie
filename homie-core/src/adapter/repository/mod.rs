use self::database::postgres::PostgresClient;
use crate::adapter::repository::database::http::HttpClient;
use crate::domain::t_yield::TYieldPersist;

pub mod database;

pub trait Persist: TYieldPersist {}

pub struct Repository {
    client: Box<dyn Persist>,
}

impl Repository {
    // Define a new function taking Config as a parameter
    pub fn new(config: &Config) -> Self {
        let client = Repository::establish_session(config);
        Repository { client }
    }

    // Method to establish session using the provided config
    fn establish_session(config: &Config) -> Box<dyn Persist> {
        if config.use_db {
            println!("Using PostgreSQL client.");
            Box::new(PostgresClient)
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
pub struct Config {
    use_db: bool,
}

impl Config {
    pub fn load_config() -> Config {
        Config { use_db: false }
    }
}
