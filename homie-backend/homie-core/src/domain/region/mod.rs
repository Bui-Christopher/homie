#[cfg(feature = "db")]
pub mod local;
#[cfg(feature = "db")]
pub mod persist;

use serde::{Deserialize, Serialize};
#[cfg(feature = "docs")]
use utoipa::ToSchema;

pub type City = String;
pub type Zipcode = String;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
#[cfg_attr(feature = "docs", derive(ToSchema))]
pub struct Region {
    pub(crate) city: City,
    pub(crate) zipcode: Zipcode,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RegionData {
    pub regions: Regions,
}

impl RegionData {
    pub fn regions(&self) -> &Regions {
        &self.regions
    }
}

pub type Regions = Vec<Region>;
pub type Zipcodes = Vec<Zipcode>;
pub type Cities = Vec<City>;

#[cfg(feature = "db")]
#[derive(Clone, Debug, Default)]
pub struct RegionQuery {
    cities: Cities,
    zipcodes: Zipcodes,
}

#[cfg(feature = "db")]
impl RegionQuery {
    pub fn new(cities: Cities, zipcodes: Zipcodes) -> Self {
        Self { cities, zipcodes }
    }

    pub(crate) fn cities(&self) -> &Cities {
        &self.cities
    }

    pub(crate) fn zipcodes(&self) -> &Zipcodes {
        &self.zipcodes
    }
}
