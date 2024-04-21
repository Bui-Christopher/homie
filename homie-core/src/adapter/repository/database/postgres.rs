use std::error::Error;

use crate::adapter::repository::Persist;
use crate::domain::hpi::{Hpi, HpiData, HpiPersist, RegionQuery};
use crate::domain::t_yield::{TYield, TYieldData, TYieldPersist, TYieldQuery};
use crate::domain::zhvi::{Zhvi, ZhviData, ZhviPersist, ZhviQuery};

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

impl Persist for PostgresClient {}

impl HpiPersist for PostgresClient {
    fn create_hpi(&self, hpi: &Hpi) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi create for: {:?} from PostgresClient.", hpi);
        Ok(true)
    }

    fn read_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_hpi(&self, hpi: &Hpi) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi update for: {:?} from PostgresClient.", hpi);
        Ok(true)
    }

    fn delete_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi delete with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn read_hpi_by_query(&self, query: &RegionQuery) -> Result<HpiData, Box<dyn Error>> {
        println!("Calling hpi read by: {:?} from PostgresClient.", query);
        Ok(HpiData::default())
    }
}

impl TYieldPersist for PostgresClient {
    fn create_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>> {
        println!(
            "Calling t_yield create for: {:?} from PostgresClient.",
            t_yield
        );
        Ok(true)
    }

    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>> {
        println!(
            "Calling t_yield update for: {:?} from PostgresClient.",
            t_yield
        );
        Ok(true)
    }

    fn delete_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield delete with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn read_t_yield_by_query(&self, query: &TYieldQuery) -> Result<TYieldData, Box<dyn Error>> {
        println!("Calling t_yield read by: {:?} from PostgresClient.", query);
        Ok(TYieldData::default())
    }
}

impl ZhviPersist for PostgresClient {
    fn create_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi create for: {:?} from PostgresClient.", zhvi);
        Ok(true)
    }

    fn read_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi update for: {:?} from PostgresClient.", zhvi);
        Ok(true)
    }

    fn delete_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi delete with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<ZhviData, Box<dyn Error>> {
        println!("Calling zhvi read by: {:?} from PostgresClient.", query);
        Ok(ZhviData::default())
    }
}
