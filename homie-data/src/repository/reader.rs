use std::error::Error;

use crate::config::Config;
use crate::model::common::Datasets;
use crate::model::hpi::read_fhfa_hpis;
use crate::model::region::read_huduser_regions;
use crate::model::t_yield::read_fed_yields;
use crate::model::zhvi::read_zillow_zhvis;

pub struct Reader {}

// config: &'static crate::config::Config,
impl Reader {
    pub fn new(_config: &'static Config) -> Self {
        Reader {}
    }
}

impl Reader {
    pub fn read_datasets(&self) -> Result<Datasets, Box<dyn Error>> {
        let t_yield_data = read_fed_yields()?;
        let hpi_data = read_fhfa_hpis()?;
        let region_data = read_huduser_regions()?;
        let zhvi_data = read_zillow_zhvis()?;

        Ok(Datasets {
            hpi_data,
            region_data,
            t_yield_data,
            zhvi_data,
        })
    }
}
