use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::adapter::repository::database::common::CRUDOperations;
use crate::config::Config;

pub(crate) mod database;

pub(crate) struct Repository<D: CRUDOperations<T>, T> {
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

    pub fn run_all_crud(&self, data: &T) -> Result<(), Box<dyn Error>> {
        self.client().create(data)?;
        self.client().read("Read Key")?;
        self.client().update(data)?;
        self.client().delete("Delete Key")?;

        println!("\n{:#?}", data);
        Ok(())
    }
}
