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

fn main() -> Result<(), Box<dyn Error>> {
    // let ten_year_yield_data = read_fed_yields()?;
    // println!("{:#?}", ten_year_yield_data);

    // let fhfa_hpi_data = read_fhfa_hpis()?;
    // println!("{:#?}", fhfa_hpi_data);

    // let regions = read_huduser_regions()?;
    // println!("{:#?}", regions);

    // TODO: Delete (for testing)
    // let tmp: Regions = regions
    //     .counties
    //     .into_iter()
    //     .filter(|region| region.city() == "IRVINE")
    //     .collect();
    // println!("{:#?}", tmp);

    let zillow_zhvi_data = read_zillow_zhvis()?;
    // println!("{:#?}", zillow_zhvi_data);

    Ok(())
}
