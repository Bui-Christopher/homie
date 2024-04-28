use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CsvRecord(pub(crate) Vec<String>);

pub(crate) fn to_ymd_date(year: u32, month: u32, day: Option<u32>) -> Result<NaiveDate, Error> {
    // If day is not present, default to 15
    let day = day.unwrap_or(15);
    let year = year as i32;
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| Error::Parse("Invalid date".to_string()))
}
