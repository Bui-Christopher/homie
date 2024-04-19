use std::error::Error;
use std::fmt::Debug;

use self::database::file::FileStorage;
use self::database::postgres::Postgres;
use crate::adapter::repository::database::common::CRUDOperations;

pub mod database;

#[derive(Debug)]
pub struct Repository<T> {
    client: Box<dyn CRUDOperations<T>>,
}

impl<T: Debug + Send + Sync> Repository<T> {
    pub fn new(config: &'static Config) -> Self {
        let client = create_client(config);
        Repository { client }
    }

    fn client(&self) -> &dyn CRUDOperations<T> {
        &*self.client
    }

    pub fn read_data(&self, data: &T) -> Result<(), Box<dyn Error>> {
        self.client().create(data)?;
        self.client().read("data")?;
        self.client().update(data)?;
        self.client().delete("data")?;

        println!("\n{:#?}", self);
        println!("\n{:#?}", data);
        Ok(())
    }

    pub fn call_all_crud(&self, data: &T) -> Result<(), Box<dyn Error>> {
        self.client().create(data)?;
        self.client().read("data")?;
        self.client().update(data)?;
        self.client().delete("data")?;

        println!("\n{:#?}", self);
        println!("\n{:#?}", data);
        Ok(())
    }
}
fn create_client<T: Debug>(config: &'static Config) -> Box<dyn CRUDOperations<T>> {
    if config.use_db {
        Box::new(Postgres::new())
    } else {
        Box::new(FileStorage::new())
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
