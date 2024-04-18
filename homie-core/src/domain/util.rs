use std::error::Error;

use chrono::NaiveDate;

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
