use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub struct TenYearYield {
    date: NaiveDate,
    yield_return: f64,
}

type TenYearYields = Vec<TenYearYield>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TreasuryYieldData {
    ten_year_yields: TenYearYields,
}

// TODO:
// impl From<Entry> for TenTreasuryYield
// Unit tests

pub fn to_ymd_date(year: u32, month: u32, day: Option<u32>) -> Result<NaiveDate, Box<dyn Error>> {
    // If day is not present, default to 15
    let day = day.unwrap_or(15);
    let year = year as i32;
    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| "Invalid date".into())
}

pub fn read_fed_yields() -> Result<TreasuryYieldData, Box<dyn Error>> {
    let ten_year_yields = read_fed_ten_yields()?;
    Ok(TreasuryYieldData { ten_year_yields })
}

fn read_fed_ten_yields() -> Result<TenYearYields, Box<dyn Error>> {
    let fed_h15 = "datasets/fed-h15/FRB_H15.csv";

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(fed_h15)?;

    // TODO: rdr.deserialize().into_iter()?.into().collect();
    let mut entries = vec![];
    let mut ten_year_yields = vec![];
    for result in rdr.deserialize() {
        let r: Entry = result?;
        entries.push(r);
    }

    for entry in entries.into_iter() {
        let parts: Vec<&str> = entry.0[0].split('-').collect();
        let year = parts[0].parse().unwrap();
        let month = parts[1].parse().unwrap();
        let date = to_ymd_date(year, month, None).unwrap();
        let yield_return = entry.0[1].parse().unwrap();
        ten_year_yields.push(TenYearYield { date, yield_return });
    }

    Ok(ten_year_yields)
}
