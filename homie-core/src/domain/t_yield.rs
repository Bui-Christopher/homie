use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::Persist;
use crate::domain::common::{to_ymd_date, CsvRecord};

// TODO: Set yield_return as option because h15 has random null fields???
#[derive(Debug, Serialize, Deserialize)]
pub enum TYield {
    TenYearYield { date: NaiveDate, yield_return: f32 },
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TYieldData {
    ten_year_yields: TYields,
}

impl TYieldData {
    pub fn ten_year_yields(&self) -> &TYields {
        &self.ten_year_yields
    }
}

pub type TYields = Vec<TYield>;

pub trait TYieldPersist: Send + Sync {
    fn create_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>>;
    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>>;
    fn update_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>>;
    fn delete_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>>;
}

impl TYield {
    pub fn create(&self, client: &dyn Persist) -> Result<bool, Box<dyn Error>> {
        client.create_t_yield(self)
    }

    pub fn read(&self, client: &dyn Persist, id: &str) -> Result<bool, Box<dyn Error>> {
        client.read_t_yield_by_id(id)
    }

    pub fn update(&self, client: &dyn Persist) -> Result<bool, Box<dyn Error>> {
        client.update_t_yield(self)
    }

    pub fn delete(&self, client: &dyn Persist, id: &str) -> Result<bool, Box<dyn Error>> {
        client.delete_t_yield_by_id(id)
    }
}

impl Default for TYield {
    fn default() -> Self {
        TYield::TenYearYield {
            date: NaiveDate::default(),
            yield_return: 0.0,
        }
    }
}

pub struct TYieldConfig {
    ten_year_yield_path: Option<String>,
}

impl TYieldConfig {
    pub fn new(ten_year_yield_path: Option<String>) -> Self {
        TYieldConfig {
            ten_year_yield_path,
        }
    }

    fn has_ten_year_yield_path(&self) -> bool {
        self.ten_year_yield_path.is_some()
    }
}

// TODO:
// impl From<Entry> for TenTreasuryYield
// Unit tests

pub fn read_fed_yields(t_yield_config: &TYieldConfig) -> Result<TYieldData, Box<dyn Error>> {
    let mut t_yield_data = TYieldData::default();
    if t_yield_config.has_ten_year_yield_path() {
        t_yield_data.ten_year_yields = read_fed_ten_yields()?;
    }
    Ok(t_yield_data)
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
        let r: CsvRecord = result?;
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
