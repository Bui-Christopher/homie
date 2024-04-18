use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::common::{to_ymd_date, Entry};

type TYields = Vec<TYield>;

// TODO: Set yield_return as option because h15 has random null fields???
#[derive(Debug, Serialize, Deserialize)]
pub enum TYield {
    TenYearYield { date: NaiveDate, yield_return: f32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TYieldData {
    pub ten_year_yields: TYields,
}

// TODO:
// impl From<Entry> for TenTreasuryYield
// Unit tests

pub fn read_fed_yields() -> Result<TYieldData, Box<dyn Error>> {
    let ten_year_yields = read_fed_ten_yields()?;
    Ok(TYieldData { ten_year_yields })
}

fn read_fed_ten_yields() -> Result<TYields, Box<dyn Error>> {
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
        ten_year_yields.push(TYield::TenYearYield { date, yield_return });
    }

    Ok(ten_year_yields)
}
