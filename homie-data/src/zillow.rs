use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::Entry;

#[derive(Debug, Serialize, Deserialize)]
struct AllHomesZHVI {}

#[derive(Debug, Serialize, Deserialize)]
struct CondoCoopsZHVI {}

#[derive(Debug, Serialize, Deserialize)]
struct SingleFamilyHomesZHVI {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZhviData {
    all_homes_zhvi: Vec<AllHomesZHVI>,
    condo_coops_zhvi: Vec<CondoCoopsZHVI>,
    single_family_homes_zhvi: Vec<SingleFamilyHomesZHVI>,
}

pub fn read_zillow_zhvi() -> Result<ZhviData, Box<dyn Error>> {
    let all_homes_zhvi = read_all_homes_zhvi()?;
    let condo_coops_zhvi = read_condo_coops_zhvi()?;
    let single_family_homes_zhvi = read_single_family_homes_zhvi()?;

    Ok(ZhviData {
        all_homes_zhvi,
        condo_coops_zhvi,
        single_family_homes_zhvi,
    })
}

fn read_all_homes_zhvi() -> Result<Vec<AllHomesZHVI>, Box<dyn Error>> {
    let zillow_mid_all_city = "datasets/zillow-zhvi/all-homes/mid-tier/\
                               City_zhvi_uc_sfrcondo_tier_0.33_0.67_sm_sa_month.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(zillow_mid_all_city)?;

    let mut _entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        _entries.push(r);
    }
    Ok(vec![])
}

fn read_condo_coops_zhvi() -> Result<Vec<CondoCoopsZHVI>, Box<dyn Error>> {
    todo!()
}

fn read_single_family_homes_zhvi() -> Result<Vec<SingleFamilyHomesZHVI>, Box<dyn Error>> {
    todo!()
}
