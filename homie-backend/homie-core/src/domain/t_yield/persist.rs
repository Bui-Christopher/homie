#[cfg(feature = "db")]
use async_trait::async_trait;
use chrono::NaiveDate;

use crate::adapter::repository::Persist;
use crate::domain::common::DateInterval;
use crate::domain::t_yield::{TYield, TYields};
use crate::error::DomainError;

#[cfg(feature = "db")]
#[async_trait]
pub trait TYieldPersist: Send + Sync {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<(String, NaiveDate), DomainError>;
    async fn read_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<TYield, DomainError>;
    async fn update_t_yield(&self, t_yield: &TYield) -> Result<(), DomainError>;
    async fn delete_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<(), DomainError>;
    async fn read_t_yields_by_query(&self, query: &TYieldQuery) -> Result<TYields, DomainError>;
}

#[cfg(feature = "db")]
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

#[cfg(feature = "db")]
#[derive(Clone, Debug, Default)]
pub struct TYieldQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
    date_interval: DateInterval,
}

#[cfg(feature = "db")]
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
