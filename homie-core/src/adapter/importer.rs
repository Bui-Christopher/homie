use std::error::Error;

use crate::adapter::repository::Config;
use crate::domain::dataset::Dataset;
use crate::domain::hpi::read_fhfa_hpis;
use crate::domain::region::read_huduser_regions;
use crate::domain::t_yield::read_fed_yields;
use crate::domain::zhvi::read_zillow_zhvis;

pub struct Importer {}

impl Importer {
    pub fn new(_config: &'static Config) -> Self {
        Importer {}
    }
}

impl Importer {
    // TODO: Abstract this: read datasets by config
    pub fn read_datasets(&self) -> Result<Dataset, Box<dyn Error>> {
        let t_yield_data = read_fed_yields()?;
        let hpi_data = read_fhfa_hpis()?;
        let region_data = read_huduser_regions()?;
        let zhvi_data = read_zillow_zhvis()?;

        Ok(Dataset {
            hpi_data,
            region_data,
            t_yield_data,
            zhvi_data,
        })
    }
}
