use std::error::Error;

use crate::common::Datasets;
use crate::config::Config;

pub struct Writer {}

impl Writer {
    pub fn new(_config: &Config) -> Self {
        Writer {}
    }
}

impl Writer {
    pub fn write_datasets(&self, _datasets: &Datasets) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
// fed_yield_data.write(db);
// fhfa_hpi_data.write(db);
// huduser_region-data.write(db);
// zillow_zhvi_data.write(db);

// fed_yield_data.write(db);
// for ten_year_yield in fed_yield_data.ten_year_yields {}
// for yield in fed_yield_data {
//     yield.write(db);
// }
// for hpi in fhfa_hpi_data {
//     hpi.write(db);
// }
// for region in huduser_region_data {
//     region.write(db);
// }
// for zhvi in zillow_zhvi_data {
//     zhvi.write(db);
// }
