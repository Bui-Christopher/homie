use std::error::Error;

use crate::adapter::repository::Config;
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
    pub fn read_and_import_datasets(&self) -> Result<(), Box<dyn Error>> {
        let _t_yield_data = read_fed_yields()?;
        let _hpi_data = read_fhfa_hpis()?;
        let _region_data = read_huduser_regions()?;
        let _zhvi_data = read_zillow_zhvis()?;
        Ok(())
    }
}
