use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::hpi::HPIData;
use crate::domain::region::RegionData;
use crate::domain::t_yield::TYieldData;
use crate::domain::zhvi::ZHVIData;

// TODO: Rename to CsvRecord
// Refactor into reader/dataset_importer
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Entry(pub(crate) Vec<String>);

// TODO: Can this be refactored to mod.rs?
#[derive(Debug, Serialize, Deserialize)]
pub struct Datasets {
    pub hpi_data: HPIData,
    pub region_data: RegionData,
    pub t_yield_data: TYieldData,
    pub zhvi_data: ZHVIData,
}

impl Datasets {
    fn new() -> Self {
        Datasets {
            hpi_data: HPIData {
                three_zip_hpis: vec![],
                five_zip_hpis: vec![],
                county_hpis: vec![],
            },
            region_data: RegionData {
                counties: vec![],
                zipcodes: vec![],
            },
            t_yield_data: TYieldData {
                ten_year_yields: vec![],
            },
            zhvi_data: ZHVIData {
                all_homes_zhvis: vec![],
                condo_coops_zhvis: vec![],
                single_family_homes_zhvis: vec![],
            },
        }
    }
}

impl Default for Datasets {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Refactor into util.rs
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
