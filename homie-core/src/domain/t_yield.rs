use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::Type;

use crate::adapter::repository::Persist;
use crate::domain::common::{to_ymd_date, CsvRecord};
use crate::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "term", rename_all = "lowercase")]
pub enum Term {
    TenYear,
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::TenYear => write!(f, "ten_year"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct TYield {
    pub(crate) term: Term,
    pub(crate) date: NaiveDate,
    pub(crate) yield_return: Option<f32>,
}

impl TYield {
    pub(crate) fn term(&self) -> &Term {
        &self.term
    }

    pub(crate) fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub(crate) fn yield_return(&self) -> &Option<f32> {
        &self.yield_return
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
pub struct TYieldQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
    interval_date: String, // Day, Month, Year
}

impl TYieldQuery {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate, interval_date: String) -> Self {
        Self {
            start_date,
            end_date,
            interval_date,
        }
    }

    pub(crate) fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    pub(crate) fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }

    pub(crate) fn interval_date(&self) -> &str {
        &self.interval_date
    }
}

#[async_trait]
pub trait TYieldPersist: Send + Sync {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<(String, NaiveDate), Error>;
    async fn read_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<TYield, Error>;
    async fn update_t_yield(&self, t_yield: &TYield) -> Result<(), Error>;
    async fn delete_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<(), Error>;
    async fn read_t_yield_by_query(&self, query: &TYieldQuery) -> Result<TYields, Error>;
}

impl TYield {
    pub async fn create(&self, client: &dyn Persist) -> Result<(String, NaiveDate), Error> {
        client.create_t_yield(self).await
    }

    pub async fn read(client: &dyn Persist, id: (&str, &NaiveDate)) -> Result<TYield, Error> {
        client.read_t_yield_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), Error> {
        client.update_t_yield(self).await
    }

    pub async fn delete(client: &dyn Persist, id: (&str, &NaiveDate)) -> Result<(), Error> {
        client.delete_t_yield_by_id(id).await
    }

    pub async fn read_by_query(
        client: &dyn Persist,
        query: &TYieldQuery,
    ) -> Result<TYields, Error> {
        client.read_t_yield_by_query(query).await
    }
}

impl Default for TYield {
    fn default() -> Self {
        TYield {
            term: Term::TenYear,
            date: NaiveDate::default(),
            yield_return: Some(0.0),
        }
    }
}

pub(crate) struct TYieldConfig {
    ten_year_yield_path: Option<String>,
}

impl TYieldConfig {
    pub(crate) fn new(ten_year_yield_path: Option<String>) -> Self {
        TYieldConfig {
            ten_year_yield_path,
        }
    }

    fn ten_year_yield_path(&self) -> Option<&str> {
        self.ten_year_yield_path.as_deref()
    }
}

// TODO:
// impl From<Entry> for TenTreasuryYield
// Unit tests

pub(crate) fn read_fed_yields(t_yield_config: &TYieldConfig) -> Result<TYieldData, Error> {
    let mut t_yield_data = TYieldData::default();
    if let Some(ten_year_yield_path) = t_yield_config.ten_year_yield_path() {
        t_yield_data.ten_year_yields = read_fed_ten_yields(ten_year_yield_path)?;
    }
    Ok(t_yield_data)
}

fn read_fed_ten_yields(fed_h15: &str) -> Result<TYields, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(fed_h15)?;

    let mut ten_year_yields = vec![];
    let entries: Vec<CsvRecord> = rdr.deserialize().filter_map(Result::ok).collect();

    for entry in entries.into_iter() {
        let parts: Vec<&str> = entry.0[0].split('-').collect();
        let year = parts[0].parse()?;
        let month = parts[1].parse()?;
        let term = Term::TenYear;
        let date = to_ymd_date(year, month, 1)?; // TODO: Random day here... Why?
        let yield_return = entry.0[1].parse().ok();
        ten_year_yields.push(TYield {
            term,
            date,
            yield_return,
        });
    }

    Ok(ten_year_yields)
}
