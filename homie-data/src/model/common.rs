use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::hpi::HPIData;
use crate::region::RegionData;
use crate::t_yield::TYieldData;
use crate::zhvi::ZHVIData;

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
