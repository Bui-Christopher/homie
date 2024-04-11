use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::Entry;

#[derive(Debug, Serialize, Deserialize)]
enum Region {
    Zipcode(String),
    City(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionData {
    zipcodes: Vec<Region>,
    cities: Vec<Region>,
}

pub fn read_huduser_region() -> Result<RegionData, Box<dyn Error>> {
    let huduser_crosswalk = "datasets/huduser-crosswalk/COUNTY_ZIP_122023.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(huduser_crosswalk)?;

    let mut _entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        _entries.push(r);
    }
    Ok(RegionData {
        zipcodes: vec![],
        cities: vec![],
    })
}
