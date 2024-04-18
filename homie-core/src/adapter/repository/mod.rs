use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::adapter::repository::database::common::CRUDOperations;

pub mod database;

pub struct Repository<D: CRUDOperations<T>, T> {
    client: D,
    marker: PhantomData<T>,
}

impl<D: CRUDOperations<T>, T: Debug> Repository<D, T> {
    pub fn new(_config: &'static Config, client: D) -> Self {
        Repository {
            client,
            marker: PhantomData,
        }
    }

    fn client(&self) -> &D {
        &self.client
    }

    pub fn write_data(&self, data: &T) -> Result<(), Box<dyn Error>> {
        self.client().create(data)?;
        self.client().read("data")?;
        self.client().update(data)?;
        self.client().delete("data")?;

        println!("\n{:#?}", data);
        Ok(())
    }
}

// TODO: Refactor out
pub struct Config {}

impl Config {
    pub fn load_config() -> Config {
        Config {}
    }
}
