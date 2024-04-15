use std::error::Error;
use std::marker::PhantomData;

use crate::database::common::CRUDOperations;

pub(crate) struct Writer<D: CRUDOperations<T>, T> {
    database_client: D,
    marker: PhantomData<T>,
}

impl<D: CRUDOperations<T>, T: std::fmt::Debug> Writer<D, T> {
    pub fn new(database_client: D) -> Self {
        Writer {
            database_client,
            marker: PhantomData,
        }
    }

    fn database_client(&self) -> &D {
        &self.database_client
    }

    pub fn write_datasets(&self, datasets: &T) -> Result<(), Box<dyn Error>> {
        self.database_client().create(datasets)?;
        self.database_client().read("Read Key")?;
        self.database_client().update(datasets)?;
        self.database_client().delete("Delete Key")?;

        println!("\n{:#?}", datasets);
        Ok(())
    }
}
