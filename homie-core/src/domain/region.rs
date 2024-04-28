use serde::{Deserialize, Serialize};

use crate::domain::common::CsvRecord;
use crate::error::Error;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionData {
    pub counties: Regions,
    pub zipcodes: Regions,
}

pub type Regions = Vec<Region>;

// TODO:
// impl From<Entry> for RegionCossData
// Unit tests

pub trait RegionPersist: Send + Sync {
    // fn create_region(&self, region: &Region) -> Result<bool,
    // Error>;
    fn read_region_by_id(&self, id: &str) -> Result<bool, Error>;
    // fn update_region(&self, region: &Region) -> Result<bool,
    // Error>; fn delete_by_id(&self, id: &str) -> Result<bool,
    // Error>;
}

pub fn read_huduser_regions() -> Result<RegionData, Error> {
    let counties = read_county_zipcodes()?;
    let zipcodes = read_zip_counties()?;

    Ok(RegionData { counties, zipcodes })
}

fn read_county_zipcodes() -> Result<Regions, Error> {
    let huduser_crosswalk = "datasets/huduser-crosswalk/COUNTY_ZIP_122023.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(huduser_crosswalk)?;

    let mut counties = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    for entry in entries.into_iter() {
        let county = entry.0[0].clone();
        let zip = entry.0[1].clone();
        let city = entry.0[2].clone();
        let res_ratio = entry.0[4].parse()?;
        let bus_ratio = entry.0[5].parse()?;
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

fn read_zip_counties() -> Result<Regions, Error> {
    let huduser_crosswalk = "datasets/huduser-crosswalk/ZIP_COUNTY_122023.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(huduser_crosswalk)?;

    let mut zipcodes = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();
    for entry in entries.into_iter() {
        let zip = entry.0[0].clone();
        let county = entry.0[1].clone();
        let city = entry.0[2].clone();
        let res_ratio = entry.0[4].parse()?;
        let bus_ratio = entry.0[5].parse()?;
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
