use std::error::Error;

use crate::adapter::repository::Persist;
use crate::domain::hpi::{Hpi, HpiData, HpiPersist, RegionQuery};
use crate::domain::t_yield::{TYield, TYieldData, TYieldPersist, TYieldQuery};
use crate::domain::zhvi::{Zhvi, ZhviData, ZhviPersist, ZhviQuery};

pub struct HttpClient;

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new()
    }
}

impl Persist for HttpClient {}

impl HpiPersist for HttpClient {
    fn create_hpi(&self, hpi: &Hpi) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi create for: {:?} from HttpClient.", hpi);
        Ok(true)
    }

    fn read_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi read with id: {id} from HttpClient.");
        Ok(true)
    }

    fn update_hpi(&self, hpi: &Hpi) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi update for: {:?} from HttpClient.", hpi);
        Ok(true)
    }

    fn delete_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi delete with id: {id} from HttpClient.");
        Ok(true)
    }

    fn read_hpi_by_query(&self, query: &RegionQuery) -> Result<HpiData, Box<dyn Error>> {
        println!("Calling hpi read by: {:?} from HttpClient.", query);
        Ok(HpiData::default())
    }
}

impl TYieldPersist for HttpClient {
    fn create_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield create for: {:?} from HttpClient.", t_yield);
        Ok(true)
    }

    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield read with id: {id} from HttpClient.");
        Ok(true)
    }

    fn update_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield update for: {:?} from HttpClient.", t_yield);
        Ok(true)
    }

    fn delete_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield delete with id: {id} from HttpClient.");
        Ok(true)
    }

    fn read_t_yield_by_query(&self, query: &TYieldQuery) -> Result<TYieldData, Box<dyn Error>> {
        println!("Calling t_yield read by: {:?} from HttpClient.", query);
        Ok(TYieldData::default())
    }
}

impl ZhviPersist for HttpClient {
    fn create_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi create for: {:?} from HttpClient.", zhvi);
        Ok(true)
    }

    fn read_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi read with id: {id} from HttpClient.");
        Ok(true)
    }

    fn update_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi update for: {:?} from HttpClient.", zhvi);
        Ok(true)
    }

    fn delete_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi delete with id: {id} from HttpClient.");
        Ok(true)
    }

    fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<ZhviData, Box<dyn Error>> {
        println!("Calling zhvi read by: {:?} from HttpClient.", query);
        Ok(ZhviData::default())
    }
}
