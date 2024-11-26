use async_trait::async_trait;

use crate::adapter::repository::Persist;
use crate::domain::region::{Region, RegionQuery, Regions, Zipcode};
use crate::error::DomainError;

#[cfg(feature = "db")]
#[async_trait]
pub trait RegionPersist: Send + Sync {
    async fn create_region(&self, region: &Region) -> Result<Zipcode, DomainError>;
    async fn read_region_by_id(&self, id: &str) -> Result<Region, DomainError>;
    async fn read_regions_by_city(&self, id: &str) -> Result<Regions, DomainError>;
    async fn read_regions_by_query(&self, query: &RegionQuery) -> Result<Regions, DomainError>;
    async fn delete_region_by_id(&self, id: &str) -> Result<Zipcode, DomainError>;
}

#[cfg(feature = "db")]
impl Region {
    pub async fn create(&self, client: &dyn Persist) -> Result<Zipcode, DomainError> {
        client.create_region(self).await
    }

    pub async fn read(client: &dyn Persist, id: &str) -> Result<Regions, DomainError> {
        client.read_regions_by_city(id).await
    }

    pub async fn read_by_query(
        client: &dyn Persist,
        query: &RegionQuery,
    ) -> Result<Regions, DomainError> {
        client.read_regions_by_query(query).await
    }

    pub async fn delete(client: &dyn Persist, id: &str) -> Result<Zipcode, DomainError> {
        client.delete_region_by_id(id).await
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn zipcode(&self) -> &str {
        &self.zipcode
    }
}
