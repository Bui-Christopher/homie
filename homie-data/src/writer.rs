use std::error::Error;
use std::marker::PhantomData;

use crate::database::common::CRUDOperations;
use crate::model::common::Datasets;

pub(crate) struct Writer<D: CRUDOperations<T>, T> {
    database_client: D,
    marker: PhantomData<T>,
}
impl<D: CRUDOperations<T>, T> Writer<D, T> {
    pub fn new(database_client: D) -> Self {
        Writer {
            database_client,
            marker: PhantomData,
        }
    }

    fn database_client(&self) -> &D {
        &self.database_client
    }

    pub fn write_datasets(&self, datasets: &Datasets) -> Result<(), Box<dyn Error>> {
        // TODO: Delete (testing)
        println!("{:#?}", self.database_client());
        println!("{:#?}", datasets);
        Ok(())
    }
}
