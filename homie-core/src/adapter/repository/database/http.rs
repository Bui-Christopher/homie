use std::error::Error;

use async_trait::async_trait;

use crate::adapter::repository::Persist;
use crate::domain::hpi::{Hpi, HpiData, HpiPersist, HpiQuery, Hpis};
use crate::domain::t_yield::{TYield, TYieldPersist, TYieldQuery, TYields};
use crate::domain::zhvi::{Zhvi, ZhviPersist, ZhviQuery, Zhvis};

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

    fn read_hpi_by_query(&self, query: &HpiQuery) -> Result<Hpis, Box<dyn Error>> {
        println!("Calling hpi read by: {:?} from HttpClient.", query);
        Ok(HpiData::generate_dummy_data())
    }
}

#[async_trait]
impl TYieldPersist for HttpClient {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>> {
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

    fn read_t_yield_by_query(&self, query: &TYieldQuery) -> Result<TYields, Box<dyn Error>> {
        println!("Calling t_yield read by: {:?} from HttpClient.", query);
        Ok(TYield::generate_dummy_data())
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

    fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<Zhvis, Box<dyn Error>> {
        println!("Calling zhvi read by: {:?} from HttpClient.", query);
        Ok(Zhvi::generate_dummy_data())
    }
}
