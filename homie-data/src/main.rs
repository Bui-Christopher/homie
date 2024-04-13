use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::fed_h15::*;
use crate::fhfa::*;
use crate::huduser::{read_huduser_regions, CountyCross};
use crate::zillow::*;

mod fed_h15;
mod fhfa;
mod huduser;
mod zillow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry(Vec<String>);

fn main() -> Result<(), Box<dyn Error>> {
    // let ten_year_yield_data = read_fed_ten_yield()?;
    // println!("{:#?}", ten_year_yield_data);

    // let fhfa_hpi_data = read_fhfa_hpi()?;
    // println!("{:#?}", fhfa_hpi_data);

    let regions = read_huduser_regions()?;
    let tmp: Vec<CountyCross> = regions
        .counties
        .into_iter()
        .filter(|region| region.city == "IRVINE")
        .collect();
    println!("{:#?}", tmp);

    // println!("{:#?}", regions.counties.counties.first().unwrap());
    // println!("{:#?}", regions.zipcodes.zipcodes.first().unwrap());

    // let zillow_zhvi_data = read_zillow_zhvi()?;
    // println!("{:#?}", zillow_zhvi_data);

    Ok(())
}
