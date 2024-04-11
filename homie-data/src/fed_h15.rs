use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub struct TenTreasuryYield {
    date: NaiveDate,
    yield_return: f64,
}

fn to_ymd_date(year: u32, month: u32, day: Option<u32>) -> Result<NaiveDate, Box<dyn Error>> {
    // Some datasets are an average for the month
    // Thus, default to the fifteenth
    let year = year as i32;
    let day = day.unwrap_or(15);
    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| "Invalid date".into())
}

pub fn read_fed_ten_yield() -> Result<Vec<TenTreasuryYield>, Box<dyn Error>> {
    let fed_h15 = "datasets/fed-h15/FRB_H15.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(fed_h15)?;

    let mut entries = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }
    Ok(entries
        .into_iter()
        .map(|entry| {
            let parts: Vec<&str> = entry.0[0].split('-').collect();
            let year = parts[0].parse().unwrap();
            let month = parts[1].parse().unwrap();
            let date = to_ymd_date(year, month, None).unwrap();
            let yield_return = entry.0[1].parse().unwrap();
            TenTreasuryYield { date, yield_return }
        })
        .collect())
}
