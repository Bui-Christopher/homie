use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::domain::common::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub enum Region {
    County {
        county: String,
        zipcode: String,
        city: String,
        // Ratio of Residential Addresses
        // Ratio of Business Addresses
        res_ratio: f64,
        bus_ratio: f64,
    },
    Zipcode {
        zipcode: String,
        county: String,
        city: String,
        // Ratio of Residential Addresses
        // Ratio of Business Addresses
        res_ratio: f64,
        bus_ratio: f64,
    },
}

// TODO: Check if needed
// impl Region {
//     pub fn city(&self) -> &str {
//         match self {
//             Region::County { city, .. } => city,
//             Region::Zipcode { city, .. } => city,
//         }
//     }
//
//     pub fn county(&self) -> &str {
//         match self {
//             Region::County { county, .. } => county,
//             Region::Zipcode { county, .. } => county,
//         }
//     }
//
//     pub fn zipcode(&self) -> &str {
//         match self {
//             Region::County { zipcode, .. } => zipcode,
//             Region::Zipcode { zipcode, .. } => zipcode,
//         }
//     }
// }

pub type Regions = Vec<Region>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionData {
    pub counties: Regions,
    pub zipcodes: Regions,
}

// TODO:
// impl From<Entry> for RegionCossData
// Unit tests

pub fn read_huduser_regions() -> Result<RegionData, Box<dyn Error>> {
    let counties = read_county_zipcodes()?;
    let zipcodes = read_zip_counties()?;

    Ok(RegionData { counties, zipcodes })
}

fn read_county_zipcodes() -> Result<Regions, Box<dyn Error>> {
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
        counties.push(Region::County {
            zipcode: zip,
            county,
            city,
            res_ratio,
            bus_ratio,
        });
    }

    Ok(counties)
}

fn read_zip_counties() -> Result<Regions, Box<dyn Error>> {
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
        zipcodes.push(Region::Zipcode {
            county,
            zipcode: zip,
            city,
            res_ratio,
            bus_ratio,
        });
    }

    Ok(zipcodes)
}
