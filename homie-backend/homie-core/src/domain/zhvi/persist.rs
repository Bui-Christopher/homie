use async_trait::async_trait;
use chrono::NaiveDate;

use crate::adapter::repository::Persist;
use crate::domain::common::DateInterval;
use crate::domain::zhvi::{HomeType, Percentile, RegionType, Zhvi, Zhvis};
use crate::error::DomainError;

#[async_trait]
pub trait ZhviPersist: Send + Sync {
    // TODO: Return Keys instead of unit type
    async fn create_zhvi(&self, zhvi: &Zhvi) -> Result<(), DomainError>;
    async fn read_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<Zhvi, DomainError>;
    async fn update_zhvi(&self, zhvi: &Zhvi) -> Result<(), DomainError>;
    async fn delete_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<(), DomainError>;
    async fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<Zhvis, DomainError>;
}

impl Zhvi {
    // Persist fn's
    pub async fn create(&self, client: &dyn Persist) -> Result<(), DomainError> {
        client.create_zhvi(self).await
    }

    pub async fn read(
        client: &dyn Persist,
        id: (&str, &str, &str, &str),
    ) -> Result<Zhvi, DomainError> {
        client.read_zhvi_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), DomainError> {
        client.update_zhvi(self).await
    }

    pub async fn delete(
        client: &dyn Persist,
        id: (&str, &str, &str, &str),
    ) -> Result<(), DomainError> {
        client.delete_zhvi_by_id(id).await
    }

    pub async fn read_by_query(
        client: &dyn Persist,
        query: &ZhviQuery,
    ) -> Result<Zhvis, DomainError> {
        client.read_zhvi_by_query(query).await
    }
}

#[cfg(feature = "db")]
#[derive(Clone, Debug, Default)]
pub struct ZhviQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
    date_interval: DateInterval,
    region_name: String,
    region_type: RegionType,
    home_type: HomeType,
    percentile: Percentile,
}

#[cfg(feature = "db")]
impl ZhviQuery {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        date_interval: DateInterval,
        region_name: String,
        region_type: RegionType,
        home_type: HomeType,
        percentile: Percentile,
    ) -> Self {
        ZhviQuery {
            start_date,
            end_date,
            date_interval,
            region_name,
            region_type,
            home_type,
            percentile,
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

    pub(crate) fn region_name(&self) -> &str {
        &self.region_name
    }

    pub(crate) fn region_type(&self) -> &RegionType {
        &self.region_type
    }

    pub(crate) fn home_type(&self) -> &HomeType {
        &self.home_type
    }

    pub(crate) fn percentile(&self) -> &Percentile {
        &self.percentile
    }
}
