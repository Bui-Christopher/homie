use std::error::Error;

use async_trait::async_trait;
use chrono::NaiveDate;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::adapter::repository::Persist;
use crate::domain::common::{to_ymd_date, CsvRecord};

// TODO: Set yield_return as option because h15 has random null fields???
// Or set as -1.0 if null instead of None
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

#[derive(Debug, Default)]
pub struct TYieldQuery {}

#[async_trait]
pub trait TYieldPersist: Send + Sync {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>>;
    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>>;
    fn update_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>>;
    fn delete_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>>;
    fn read_t_yield_by_query(&self, query: &TYieldQuery) -> Result<TYields, Box<dyn Error>>;
}

impl TYield {
    pub async fn create(&self, client: &dyn Persist) -> Result<bool, Box<dyn Error>> {
        client.create_t_yield(self).await
    }

    pub fn read(client: &dyn Persist, id: &str) -> Result<bool, Box<dyn Error>> {
        client.read_t_yield_by_id(id)
    }

    pub fn update(&self, client: &dyn Persist) -> Result<bool, Box<dyn Error>> {
        client.update_t_yield(self)
    }

    pub fn delete(client: &dyn Persist, id: &str) -> Result<bool, Box<dyn Error>> {
        client.delete_t_yield_by_id(id)
    }

    pub fn read_by_query(
        client: &dyn Persist,
        query: &TYieldQuery,
    ) -> Result<TYields, Box<dyn Error>> {
        client.read_t_yield_by_query(query)
    }

    // TODO: Delete
    pub fn generate_dummy_data() -> Vec<TYield> {
        let mut rng = rand::thread_rng();
        let mut dummy_data = Vec::new();

        // Generate dummy data for TenYearYield variant
        for _ in 0..1 {
            let date = NaiveDate::from_ymd_opt(
                rng.gen_range(2020..=2020),
                rng.gen_range(1..=12),
                rng.gen_range(1..=28),
            )
            .unwrap();
            let yield_return = rng.gen::<f32>();
            let ten_year_yield = TYield::TenYearYield { date, yield_return };
            dummy_data.push(ten_year_yield);
        }

        dummy_data
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

    fn ten_year_yield_path(&self) -> &str {
        self.ten_year_yield_path.as_ref().unwrap()
    }
}

// TODO:
// impl From<Entry> for TenTreasuryYield
// Unit tests

pub fn read_fed_yields(t_yield_config: &TYieldConfig) -> Result<TYieldData, Box<dyn Error>> {
    let mut t_yield_data = TYieldData::default();
    if t_yield_config.has_ten_year_yield_path() {
        t_yield_data.ten_year_yields = read_fed_ten_yields(t_yield_config.ten_year_yield_path())?;
    }
    Ok(t_yield_data)
}

fn read_fed_ten_yields(fed_h15: &str) -> Result<TYields, Box<dyn Error>> {
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
