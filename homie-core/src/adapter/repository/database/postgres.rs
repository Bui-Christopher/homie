use crate::adapter::repository::Persist;
use crate::domain::hpi::{HpiPersist, RegionHPI};
use crate::domain::t_yield::{TYield, TYieldPersist};
use crate::domain::zhvi::{Zhvi, ZhviPersist};

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
    fn create_hpi(&self, hpi: &RegionHPI) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling hpi create for: {:?} from PostgresClient.", hpi);
        Ok(true)
    }

    fn read_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling hpi read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_hpi(&self, hpi: &RegionHPI) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling hpi update for: {:?} from PostgresClient.", hpi);
        Ok(true)
    }

    fn delete_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling hpi delete with id: {id} from PostgresClient.");
        Ok(true)
    }
}

impl TYieldPersist for PostgresClient {
    fn create_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn std::error::Error>> {
        println!(
            "Calling t_yield create for: {:?} from PostgresClient.",
            t_yield
        );
        Ok(true)
    }

    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling t_yield read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn std::error::Error>> {
        println!(
            "Calling t_yield update for: {:?} from PostgresClient.",
            t_yield
        );
        Ok(true)
    }

    fn delete_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling t_yield delete with id: {id} from PostgresClient.");
        Ok(true)
    }
}

impl ZhviPersist for PostgresClient {
    fn create_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling zhvi create for: {:?} from PostgresClient.", zhvi);
        Ok(true)
    }

    fn read_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling zhvi read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling zhvi update for: {:?} from PostgresClient.", zhvi);
        Ok(true)
    }

    fn delete_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("Calling zhvi delete with id: {id} from PostgresClient.");
        Ok(true)
    }
}
