use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::Persist;
use crate::domain::common::{to_ymd_date, CsvRecord};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    date: NaiveDate,
    price: f64,
}

pub type Prices = Vec<Price>;
pub type Zhvis = Vec<Zhvi>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ZHVIData {
    all_homes_zhvis: Zhvis,
    condo_coops_zhvis: Zhvis,
    single_family_homes_zhvis: Zhvis,
}

impl ZHVIData {
    pub fn all_homes_zhvis(&self) -> &Zhvis {
        &self.all_homes_zhvis
    }

    pub fn condo_coops_zhvis(&self) -> &Zhvis {
        &self.condo_coops_zhvis
    }

    pub fn single_family_homes_zhvis(&self) -> &Zhvis {
        &self.single_family_homes_zhvis
    }
}

pub trait ZhviPersist: Send + Sync {
    fn create_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>>;
    fn read_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>>;
    fn update_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>>;
    fn delete_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>>;
}

impl Zhvi {
    pub fn create(&self, client: &dyn Persist) -> Result<bool, Box<dyn Error>> {
        client.create_zhvi(self)
    }

    pub fn read(&self, client: &dyn Persist, id: &str) -> Result<bool, Box<dyn Error>> {
        client.read_zhvi_by_id(id)
    }

    pub fn update(&self, client: &dyn Persist) -> Result<bool, Box<dyn Error>> {
        client.update_zhvi(self)
    }

    pub fn delete(&self, client: &dyn Persist, id: &str) -> Result<bool, Box<dyn Error>> {
        client.delete_zhvi_by_id(id)
    }
}

// TODO:
// impl Zhvi {
//     fn from_entry_to_all_homes() -> Self {},
//     fn from_entry_to_condo_coops() -> Self {},
//     fn from_entry_to_single_family_homes() -> Self {},
// }
// Unit Tests

pub struct ZhviConfig {
    mid_zip_all_homes_path: Option<String>,
    mid_city_all_homes_path: Option<String>,
    mid_county_all_homes_path: Option<String>,
}

impl ZhviConfig {
    pub fn new(
        mid_zip_all_homes_path: Option<String>,
        mid_city_all_homes_path: Option<String>,
        mid_county_all_homes_path: Option<String>,
    ) -> Self {
        ZhviConfig {
            mid_zip_all_homes_path,
            mid_city_all_homes_path,
            mid_county_all_homes_path,
        }
    }

    fn has_mid_zip_all_homes_path(&self) -> bool {
        self.mid_zip_all_homes_path.is_some()
    }

    fn has_mid_city_all_homes_path(&self) -> bool {
        self.mid_city_all_homes_path.is_some()
    }

    fn has_mid_county_all_homes_path(&self) -> bool {
        self.mid_county_all_homes_path.is_some()
    }
}

//     if hpi_config.has_county_hpi_path() {
//         hpi_data.county_hpis = read_county_fhfa_hpis()?;
//     }
pub fn read_zillow_zhvis(zhvi_config: &ZhviConfig) -> Result<ZHVIData, Box<dyn Error>> {
    let zhvi_data = ZHVIData {
        all_homes_zhvis: read_all_homes_zhvis(zhvi_config)?,
        ..Default::default()
    };
    // let condo_coops_zhvis = read_condo_coops_zhvis()?;
    // let single_family_homes_zhvis = read_single_family_homes_zhvis()?;

    Ok(zhvi_data)
}

fn read_all_homes_zhvis(zhvi_config: &ZhviConfig) -> Result<Zhvis, Box<dyn Error>> {
    let mut all_homes = Zhvis::default();
    if zhvi_config.has_mid_zip_all_homes_path() {
        all_homes.append(&mut read_mid_zip_all_homes()?);
    }
    if zhvi_config.has_mid_city_all_homes_path() {
        all_homes.append(&mut read_mid_city_all_homes()?);
    }
    if zhvi_config.has_mid_county_all_homes_path() {
        all_homes.append(&mut read_mid_county_all_homes()?);
    }
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
        let r: CsvRecord = result?;
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
        let r: CsvRecord = result?;
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
        let r: CsvRecord = result?;
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
