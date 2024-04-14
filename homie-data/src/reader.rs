use std::error::Error;

use crate::common::Datasets;
use crate::config::Config;
use crate::{read_fed_yields, read_fhfa_hpis, read_huduser_regions, read_zillow_zhvis};

pub struct Reader {}

impl Reader {
    pub fn new(_config: &Config) -> Self {
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
