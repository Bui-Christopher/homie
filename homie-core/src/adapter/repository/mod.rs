use self::database::postgres::PostgresClient;
use crate::adapter::config::Config;
use crate::adapter::repository::database::http::HttpClient;
use crate::domain::hpi::HpiPersist;
use crate::domain::region::RegionPersist;
use crate::domain::t_yield::TYieldPersist;
use crate::domain::zhvi::ZhviPersist;
use crate::error::Error;

pub mod database;

pub trait Persist: HpiPersist + RegionPersist + TYieldPersist + ZhviPersist {}

pub struct Repository {
    client: Box<dyn Persist>,
}

impl Repository {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let client = Repository::establish_session(config).await?;
        Ok(Repository { client })
    }

    async fn establish_session(config: &Config) -> Result<Box<dyn Persist>, Error> {
        if config.is_zillow_api_enabled() {
            println!("Using HTTP client.");
            Ok(Box::new(HttpClient))
        } else {
            Ok(Box::new(PostgresClient::new(config).await?))
        }
    }

    pub fn session(&self) -> &dyn Persist {
        &*self.client
    }
}
