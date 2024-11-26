use async_trait::async_trait;

use crate::adapter::repository::Persist;
use crate::domain::hpi::{Hpi, HpiQuery, Hpis};
use crate::error::DomainError;

#[cfg(feature = "db")]
#[async_trait]
pub trait HpiPersist: Send + Sync {
    async fn create_hpi(&self, hpi: &Hpi) -> Result<(String, i32), DomainError>;
    async fn read_hpi_by_id(&self, id: (&str, i32)) -> Result<Hpi, DomainError>;
    async fn update_hpi(&self, hpi: &Hpi) -> Result<(), DomainError>;
    async fn delete_hpi_by_id(&self, id: (&str, i32)) -> Result<(), DomainError>;
    async fn read_hpi_by_query(&self, query: &HpiQuery) -> Result<Hpis, DomainError>;
}

#[cfg(feature = "db")]
impl Hpi {
    pub async fn create(&self, client: &dyn Persist) -> Result<(String, i32), DomainError> {
        client.create_hpi(self).await
    }

    pub async fn read(client: &dyn Persist, id: (&str, i32)) -> Result<Hpi, DomainError> {
        client.read_hpi_by_id(id).await
    }

    pub async fn update(&self, client: &dyn Persist) -> Result<(), DomainError> {
        client.update_hpi(self).await
    }

    pub async fn delete(client: &dyn Persist, id: (&str, i32)) -> Result<(), DomainError> {
        client.delete_hpi_by_id(id).await
    }

    pub async fn read_by_query(
        client: &dyn Persist,
        query: &HpiQuery,
    ) -> Result<Hpis, DomainError> {
        client.read_hpi_by_query(query).await
    }
}
