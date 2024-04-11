use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::fed_h15::*;
use crate::fhfa::*;
use crate::huduser::read_huduser_region;
use crate::zillow::*;

mod fed_h15;
mod fhfa;
mod huduser;
mod zillow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry(Vec<String>);

fn main() -> Result<(), Box<dyn Error>> {
    let zillow_zhvi_data = read_zillow_zhvi()?;
    println!("{:#?}", zillow_zhvi_data);

    let fhfa_hpi_data = read_fhfa_hpi()?;
    println!("{:#?}", fhfa_hpi_data);

    let ten_year_yield_data = read_fed_ten_yield()?;
    println!("{:#?}", ten_year_yield_data);

    let regions = read_huduser_region()?;
    println!("{:#?}", regions);

    Ok(())
}
