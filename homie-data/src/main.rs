use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::fed_h15::*;
use crate::fhfa::*;
use crate::huduser::*;
use crate::zillow::*;

mod fed_h15;
mod fhfa;
mod huduser;
mod zillow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry(Vec<String>);

// struct Config {
//     read_from: DatasetType
//     write_to: Repository
// }

fn main() -> Result<(), Box<dyn Error>> {
    // setup_config?;
    // Reader::new(config)?;
    // Writer::new(config)?;
    // let datasets = Reader.read_datasets()?;
    // Writer.write_datasets(datasets)?;

    let fed_yield_data = read_fed_yields()?;
    println!("{:#?}", fed_yield_data);

    let fhfa_hpi_data = read_fhfa_hpis()?;
    println!("{:#?}", fhfa_hpi_data);

    let huduser_region_data = read_huduser_regions()?;
    println!("{:#?}", huduser_region_data);

    let zillow_zhvi_data = read_zillow_zhvis()?;
    println!("{:#?}", zillow_zhvi_data);

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

    Ok(())
}

// TODO: Delete
// Notes for later:
// let tmp: Regions = regions
//     .counties
//     .into_iter()
//     .filter(|region| region.city() == "IRVINE")
//     .collect();
// println!("{:#?}", tmp);
