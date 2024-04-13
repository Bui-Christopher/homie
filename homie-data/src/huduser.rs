use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub struct CountyCross {
    pub county: String,
    pub zip: String,
    pub city: String,
    pub res_ratio: f64, // Ratio of Residential Addresses
    pub bus_ratio: f64, // Ratio of Business Addresses
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZipCross {
    zip: String,
    county: String,
    city: String,
    pub res_ratio: f64, // Ratio of Residential Addresses
    pub bus_ratio: f64, // Ratio of Business Addresses
}

type Counties = Vec<CountyCross>;
type Zipcodes = Vec<ZipCross>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionCrossData {
    pub counties: Counties,
    pub zipcodes: Zipcodes,
}

// TODO: From<Entry> for RegionCossData

pub fn read_huduser_regions() -> Result<RegionCrossData, Box<dyn Error>> {
    let counties = read_county_zipcodes()?;
    let zipcodes = read_zip_counties()?;

    Ok(RegionCrossData { counties, zipcodes })
}

pub fn read_county_zipcodes() -> Result<Counties, Box<dyn Error>> {
    let huduser_crosswalk = "datasets/huduser-crosswalk/COUNTY_ZIP_122023.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(huduser_crosswalk)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    let mut counties = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }

    for entry in entries.into_iter() {
        let county = entry.0[0].clone();
        let zip = entry.0[1].clone();
        let city = entry.0[2].clone();
        let res_ratio = entry.0[4].parse().unwrap();
        let bus_ratio = entry.0[5].parse().unwrap();
        counties.push(CountyCross {
            zip,
            county,
            city,
            res_ratio,
            bus_ratio,
        });
    }

    Ok(counties)
}

pub fn read_zip_counties() -> Result<Zipcodes, Box<dyn Error>> {
    let huduser_crosswalk = "datasets/huduser-crosswalk/ZIP_COUNTY_122023.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(huduser_crosswalk)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    let mut zipcodes = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }

    for entry in entries.into_iter() {
        let zip = entry.0[0].clone();
        let county = entry.0[1].clone();
        let city = entry.0[2].clone();
        let res_ratio = entry.0[4].parse().unwrap();
        let bus_ratio = entry.0[5].parse().unwrap();
        zipcodes.push(ZipCross {
            county,
            zip,
            city,
            res_ratio,
            bus_ratio,
        });
    }

    Ok(zipcodes)
}
