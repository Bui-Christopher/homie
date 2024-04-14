use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::database::common::CRUDOperations;
use crate::model::hpi::HPIData;
use crate::model::region::RegionData;
use crate::model::t_yield::TYieldData;
use crate::model::zhvi::ZHVIData;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Entry(pub(crate) Vec<String>);

pub(crate) fn to_ymd_date(
    year: u32,
    month: u32,
    day: Option<u32>,
) -> Result<NaiveDate, Box<dyn Error>> {
    // If day is not present, default to 15
    let day = day.unwrap_or(15);
    let year = year as i32;
    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| "Invalid date".into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datasets {
    pub hpi_data: HPIData,
    pub region_data: RegionData,
    pub t_yield_data: TYieldData,
    pub zhvi_data: ZHVIData,
}

// impl Datasets {
//     fn new() -> Self {
//         Datasets {
//             hpi_data: HPIData { three_zip_hpis: vec![], five_zip_hpis:
// vec![], county_hpis: vec![] },             region_data: RegionData {
// counties: vec![] zipcodes: vec![] },             t_yield_data: TYieldData {
// ten_year_yields: vec![] },             zhvi_data: ZHVIData { all_homes_zhvis:
// vec![], condo_coops_zhvis: vec![], single_family_homes_zhvis: vec![] },
//         }
//     }
// }
//
// impl Default for Datasets {
//     fn default() -> Self {
//         Self::new()
//     }
// }

pub trait Persistence<T, D: CRUDOperations<T>> {
    fn create(&self, db: &D) -> Result<bool, Box<dyn Error>>;
    fn read(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>>;
    fn update(&self, db: &D, obj: &T) -> Result<bool, Box<dyn Error>>;
    fn delete(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>>;
}

impl<D: CRUDOperations<Datasets>> Persistence<Datasets, D> for Datasets {
    fn create(&self, db: &D) -> Result<bool, Box<dyn Error>> {
        db.create(self)
    }

    fn read(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>> {
        db.read(key)
    }

    fn update(&self, db: &D, obj: &Datasets) -> Result<bool, Box<dyn Error>> {
        db.update(obj)
    }

    fn delete(&self, db: &D, key: &str) -> Result<bool, Box<dyn Error>> {
        db.delete(key)
    }
}
