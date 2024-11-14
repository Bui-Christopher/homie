use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::DateInterval;
use crate::adapter::repository::Persist;
use crate::domain::util::{to_ymd_date, CsvRecord};
use crate::error::DomainError;

#[derive(Clone, Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "term", rename_all = "lowercase")]
pub enum Term {
    #[default]
    TenYear,
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::TenYear => write!(f, "ten_year"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TYieldData {
    ten_year_yields: TYields,
}

impl TYieldData {
    pub fn ten_year_yields(&self) -> &TYields {
        &self.ten_year_yields
    }
}

pub type TYields = Vec<TYield>;

#[derive(Clone, Debug, Default)]
pub struct TYieldQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
    date_interval: DateInterval,
}

impl TYieldQuery {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate, date_interval: DateInterval) -> Self {
        Self {
            start_date,
            end_date,
            date_interval,
        }
    }

    pub(crate) fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    pub(crate) fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }

    pub(crate) fn date_interval(&self) -> &DateInterval {
        &self.date_interval
    }
}

#[async_trait]
pub trait TYieldPersist: Send + Sync {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<(String, NaiveDate), DomainError>;
    async fn read_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<TYield, DomainError>;
    async fn update_t_yield(&self, t_yield: &TYield) -> Result<(), DomainError>;
    async fn delete_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<(), DomainError>;
    async fn read_t_yields_by_query(&self, query: &TYieldQuery) -> Result<TYields, DomainError>;
}

impl TYield {
    pub async fn create(&self, client: &dyn Persist) -> Result<(String, NaiveDate), DomainError> {
        client.create_t_yield(self).await
    }

    pub async fn read(client: &dyn Persist, id: (&str, &NaiveDate)) -> Result<TYield, DomainError> {
        client.read_t_yield_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), DomainError> {
        client.update_t_yield(self).await
    }

    pub async fn delete(client: &dyn Persist, id: (&str, &NaiveDate)) -> Result<(), DomainError> {
        client.delete_t_yield_by_id(id).await
    }

    pub async fn read_by_query(
        client: &dyn Persist,
        query: &TYieldQuery,
    ) -> Result<TYields, DomainError> {
        client.read_t_yields_by_query(query).await
    }
}

#[derive(Clone, Debug, Default)]
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

pub(crate) fn read_fed_yields(t_yield_config: &TYieldConfig) -> Result<TYieldData, DomainError> {
    let mut t_yield_data = TYieldData::default();
    if let Some(ten_year_yield_path) = t_yield_config.ten_year_yield_path() {
        t_yield_data.ten_year_yields = read_fed_ten_yields(ten_year_yield_path)?;
    }
    Ok(t_yield_data)
}

fn read_fed_ten_yields(fed_h15: &str) -> Result<TYields, DomainError> {
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
