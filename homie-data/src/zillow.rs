use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::Entry;

struct AllHomesZHVI {}

#[derive(Debug, Serialize, Deserialize)]
struct AllHomesZHVIs {
    all_homes_zhvis: Vec<AllHomesZHVI>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CondoCoopZHVI {}

#[derive(Debug, Serialize, Deserialize)]
struct SingleFamilyHomeZHVI {}

#[derive(Debug, Serialize, Deserialize)]
struct AllHomes {
    all_homes_low_zhvis: AllHomesZHVIs,
    all_homes_mid_zhvis: AllHomesZHVIs,
    all_homes_high_zhvis: AllHomesZHVIs,
}
type CondoCoops = Vec<CondoCoopZHVI>;
type SingleFamilyHomes = Vec<SingleFamilyHomeZHVI>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZhviData {
    all_homes_zhvis: AllHomes,
    condo_coops_zhvis: CondoCoops,
    single_family_homes_zhvis: SingleFamilyHomes,
}
// TODO: From<Entry> for AllHomesZHVI
// TODO: From<Entry> for CondoCoopZHVI
// TODO: From<Entry> for SingleFamilyHomeZHVI

pub fn read_zillow_zhvis() -> Result<ZhviData, Box<dyn Error>> {
    let all_homes_zhvis = read_all_homes_zhvis()?;
    let condo_coops_zhvis = read_condo_coops_zhvis()?;
    let single_family_homes_zhvis = read_single_family_homes_zhvis()?;

    Ok(ZhviData {
        all_homes_zhvis,
        condo_coops_zhvis,
        single_family_homes_zhvis,
    })
}

// fn read_all_homes_zhvis() -> Result<Vec<AllHomeZHVI>, Box<dyn Error>> {}
fn read_all_homes_zhvis() -> Result<Vec<AllHomeZHVI>, Box<dyn Error>> {
    let zillow_mid_all_city = "datasets/zillow-zhvi/all-homes/mid-tier/\
                               City_zhvi_uc_sfrcondo_tier_0.33_0.67_sm_sa_month.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(zillow_mid_all_city)?;
    // TODO: rdr.deserialize().into_iter()?.into().collect();

    let mut _entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        _entries.push(r);
    }

    Ok(vec![])
}

fn read_condo_coops_zhvis() -> Result<Vec<CondoCoopZHVI>, Box<dyn Error>> {
    todo!()
}

fn read_single_family_homes_zhvis() -> Result<Vec<SingleFamilyHomeZHVI>, Box<dyn Error>> {
    todo!()
}
