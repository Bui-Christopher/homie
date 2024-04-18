use std::error::Error;

use homie_core::domain::common::Datasets;
use homie_core::domain::hpi::read_fhfa_hpis;
use homie_core::domain::region::read_huduser_regions;
use homie_core::domain::t_yield::read_fed_yields;
use homie_core::domain::zhvi::read_zillow_zhvis;

use crate::config::Config;

pub struct Importer {}

impl Importer {
    pub fn new(_config: &'static Config) -> Self {
        Importer {}
    }
}

impl Importer {
    // TODO: Abstract this: read datasets by config
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
