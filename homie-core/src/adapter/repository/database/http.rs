use async_trait::async_trait;
use chrono::NaiveDate;

use crate::adapter::repository::Persist;
use crate::domain::hpi::{Hpi, HpiPersist, HpiQuery, Hpis};
use crate::domain::region::{Region, RegionPersist, RegionQuery, Regions, Zipcode};
use crate::domain::t_yield::{TYield, TYieldPersist, TYieldQuery, TYields};
use crate::domain::zhvi::{Zhvi, ZhviPersist, ZhviQuery, Zhvis};
use crate::error::Error;

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

#[async_trait]
impl HpiPersist for HttpClient {
    async fn create_hpi(&self, hpi: &Hpi) -> Result<(String, i32), Error> {
        println!("Calling hpi create for: {:?} from HttpClient.", hpi);
        Ok((String::default(), i32::default()))
    }

    async fn read_hpi_by_id(&self, id: (&str, i32)) -> Result<Hpi, Error> {
        println!("Calling hpi read with id: {:?} from HttpClient.", id);
        Ok(Hpi::default())
    }

    async fn update_hpi(&self, hpi: &Hpi) -> Result<(), Error> {
        println!("Calling hpi update for: {:?} from HttpClient.", hpi);
        Ok(())
    }

    async fn delete_hpi_by_id(&self, id: (&str, i32)) -> Result<(), Error> {
        println!("Calling hpi delete with id: {:?} from HttpClient.", id);
        Ok(())
    }

    async fn read_hpi_by_query(&self, query: &HpiQuery) -> Result<Hpis, Error> {
        println!("Calling hpi read by: {:?} from HttpClient.", query);
        Ok(vec![Hpi::default()])
    }
}

#[async_trait]
impl RegionPersist for HttpClient {
    async fn create_region(&self, region: &Region) -> Result<Zipcode, Error> {
        println!("Calling region create for: {:?} from HttpClient.", region);
        Ok(Zipcode::default())
    }

    async fn read_region_by_id(&self, id: &str) -> Result<Region, Error> {
        println!("Calling region read for: {:?} from HttpClient.", id);
        Ok(Region::default())
    }

    async fn read_regions_by_city(&self, id: &str) -> Result<Regions, Error> {
        println!("Calling region read with id: {:?} from HttpClient.", id);
        Ok(Regions::default())
    }

    async fn read_regions_by_query(&self, query: &RegionQuery) -> Result<Regions, Error> {
        println!(
            "Calling region read with query: {:?} from HttpClient.",
            query
        );
        Ok(Regions::default())
    }

    async fn delete_region_by_id(&self, id: &str) -> Result<Zipcode, Error> {
        println!("Calling region delete with id: {:?} from HttpClient.", id);
        Ok(Zipcode::default())
    }
}

#[async_trait]
impl TYieldPersist for HttpClient {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<(String, NaiveDate), Error> {
        println!("Calling t_yield create for: {:?} from HttpClient.", t_yield);
        Ok((String::default(), NaiveDate::default()))
    }

    async fn read_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<TYield, Error> {
        println!("Calling t_yield read with id: {:?} from HttpClient.", id);
        Ok(TYield::default())
    }

    async fn update_t_yield(&self, t_yield: &TYield) -> Result<(), Error> {
        println!("Calling t_yield update for: {:?} from HttpClient.", t_yield);
        Ok(())
    }

    async fn delete_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<(), Error> {
        println!("Calling t_yield delete with id: {:?} from HttpClient.", id);
        Ok(())
    }

    async fn read_t_yields_by_query(&self, query: &TYieldQuery) -> Result<TYields, Error> {
        println!("Calling t_yield read by: {:?} from HttpClient.", query);
        Ok(vec![TYield::default()])
    }
}

#[async_trait]
impl ZhviPersist for HttpClient {
    async fn create_zhvi(&self, zhvi: &Zhvi) -> Result<(), Error> {
        println!("Calling zhvi create for: {:?} from HttpClient.", zhvi);
        Ok(())
    }

    async fn read_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<Zhvi, Error> {
        println!("Calling zhvi read with id: {id:?} from HttpClient.");
        Ok(Zhvi::default())
    }

    async fn update_zhvi(&self, zhvi: &Zhvi) -> Result<(), Error> {
        println!("Calling zhvi update for: {:?} from HttpClient.", zhvi);
        Ok(())
    }

    async fn delete_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<(), Error> {
        println!("Calling zhvi delete with id: {id:?} from HttpClient.");
        Ok(())
    }

    async fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<Zhvis, Error> {
        println!("Calling zhvi read by: {:?} from HttpClient.", query);
        Ok(Zhvis::default())
    }
}
