use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::model::common::{to_ymd_date, Entry};

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    date: NaiveDate,
    price: f64,
}

type Prices = Vec<Price>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Region {
    Zipcode(String),
    City(String),
    County(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Percentile {
    Bottom,
    Middle,
    Top,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Zhvi {
    AllHomes {
        prices: Prices,
        region: Region,
        percentile: Percentile,
    },
    CondoCoops {
        prices: Prices,
        region: Region,
        percentile: Percentile,
    },
    SingleFamilyHomes {
        prices: Prices,
        region: Region,
        percentile: Percentile,
    },
}

type Zhvis = Vec<Zhvi>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZHVIData {
    all_homes_zhvis: Zhvis,
    condo_coops_zhvis: Zhvis,
    single_family_homes_zhvis: Zhvis,
}

// TODO:
// impl Zhvi {
//     fn from_entry_to_all_homes() -> Self {},
//     fn from_entry_to_condo_coops() -> Self {},
//     fn from_entry_to_single_family_homes() -> Self {},
// }
// Unit Tests

pub fn read_zillow_zhvis() -> Result<ZHVIData, Box<dyn Error>> {
    let all_homes_zhvis = read_all_homes_zhvis()?;
    // let condo_coops_zhvis = read_condo_coops_zhvis()?;
    // let single_family_homes_zhvis = read_single_family_homes_zhvis()?;

    // TODO: Delete (for testing)
    let condo_coops_zhvis = vec![];
    let single_family_homes_zhvis = vec![];

    Ok(ZHVIData {
        all_homes_zhvis,
        condo_coops_zhvis,
        single_family_homes_zhvis,
    })
}

fn read_all_homes_zhvis() -> Result<Zhvis, Box<dyn Error>> {
    let mut all_homes = vec![];
    let mut mid_city_all_homes = read_mid_city_all_homes()?;
    let mut mid_county_all_homes = read_mid_county_all_homes()?;
    let mut mid_zip_all_homes = read_mid_zip_all_homes()?;

    // TODO: Refactor into unit test
    // println!("Mid City: First {:?}", mid_city_all_homes.first().unwrap());
    // println!("Mid City: Last {:?}", mid_city_all_homes.last().unwrap());
    //
    // println!(
    //     "Mid County: First {:?}",
    //     mid_county_all_homes.first().unwrap()
    // );
    // println!(
    //     "Mid County: Last {:?}",
    //     mid_county_all_homes.last().unwrap()
    // );
    //
    // println!(
    //     "Mid Zipcode: First {:?}",
    //     mid_zip_all_homes.first().unwrap()
    // );
    // println!("Mid Zipcode: Last {:?}", mid_zip_all_homes.last().unwrap());

    all_homes.append(&mut mid_city_all_homes);
    all_homes.append(&mut mid_county_all_homes);
    all_homes.append(&mut mid_zip_all_homes);
    Ok(all_homes)
}

fn read_mid_city_all_homes() -> Result<Zhvis, Box<dyn Error>> {
    let city_mid_file = "datasets/zillow-zhvi/all-homes/mid-tier/City_zhvi_uc_sfrcondo_tier_0.\
                         33_0.67_sm_sa_month.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(city_mid_file)?;
    // TODO: rdr.deserialize().into_iter()?.into().collect();

    let mut entries = vec![];
    let mut mid_all_homes = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    let headers = rdr.headers()?;

    for entry in entries.into_iter() {
        // start at 8
        let mut prices = vec![];
        for i in 8..entry.0.len() {
            let parts: Vec<&str> = headers.iter().nth(i).unwrap().split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let day = parts[2].parse().unwrap();
            let date = to_ymd_date(year, month, Some(day)).unwrap();
            let price = entry.0[i].parse().unwrap_or_default();
            prices.push(Price { date, price });
        }
        let region = Region::City(entry.0[3].clone());
        mid_all_homes.push(Zhvi::AllHomes {
            prices,
            region,
            percentile: Percentile::Middle,
        });
    }

    Ok(mid_all_homes)
}

fn read_mid_county_all_homes() -> Result<Zhvis, Box<dyn Error>> {
    let county_mid_file = "datasets/zillow-zhvi/all-homes/mid-tier/County_zhvi_uc_sfrcondo_tier_0.\
                           33_0.67_sm_sa_month.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(county_mid_file)?;
    // TODO: rdr.deserialize().into_iter()?.into().collect();

    let mut entries = vec![];
    let mut mid_all_homes = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    let headers = rdr.headers()?;

    for entry in entries.into_iter() {
        // start at 8
        let mut prices = vec![];
        for i in 9..entry.0.len() {
            let parts: Vec<&str> = headers.iter().nth(i).unwrap().split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let day = parts[2].parse().unwrap();
            let date = to_ymd_date(year, month, Some(day)).unwrap();
            let price = entry.0[i].parse().unwrap_or_default();
            prices.push(Price { date, price });
        }
        let region = Region::County(entry.0[3].clone());
        mid_all_homes.push(Zhvi::AllHomes {
            prices,
            region,
            percentile: Percentile::Middle,
        });
    }

    Ok(mid_all_homes)
}

fn read_mid_zip_all_homes() -> Result<Zhvis, Box<dyn Error>> {
    let zip_mid_file = "datasets/zillow-zhvi/all-homes/mid-tier/Zip_zhvi_uc_sfrcondo_tier_0.33_0.\
                        67_sm_sa_month.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(zip_mid_file)?;
    // TODO: rdr.deserialize().into_iter()?.into().collect();

    let mut entries = vec![];
    let mut mid_all_homes = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    let headers = rdr.headers()?;

    for entry in entries.into_iter() {
        // start at 8
        let mut prices = vec![];
        for i in 9..entry.0.len() {
            let parts: Vec<&str> = headers.iter().nth(i).unwrap().split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let day = parts[2].parse().unwrap();
            let date = to_ymd_date(year, month, Some(day)).unwrap();
            let price = entry.0[i].parse().unwrap_or_default();
            prices.push(Price { date, price });
        }
        let region = Region::Zipcode(entry.0[3].clone());
        mid_all_homes.push(Zhvi::AllHomes {
            prices,
            region,
            percentile: Percentile::Middle,
        });
    }

    Ok(mid_all_homes)
}

// fn read_condo_coops_zhvis() -> Result<Zhvis, Box<dyn Error>> {
//     todo!()
// }
//
// fn read_single_family_homes_zhvis() -> Result<Zhvis, Box<dyn Error>> {
//     todo!()
// }
