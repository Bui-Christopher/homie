#[cfg(feature = "db")]
pub mod local;
#[cfg(feature = "db")]
pub mod persist;
use serde::{Deserialize, Serialize};

use super::common::RegionType;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
pub struct Hpi {
    pub(crate) region_type: RegionType,
    pub(crate) region_name: String,
    pub(crate) year: i32,
    pub(crate) hpi: Option<f32>,
    pub(crate) annual_change: Option<f32>,
    pub(crate) hpi_1990_base: Option<f32>,
    pub(crate) hpi_2000_base: Option<f32>,
}

#[cfg(feature = "db")]
impl Hpi {
    pub(crate) fn region_type(&self) -> &RegionType {
        &self.region_type
    }

    pub(crate) fn region_name(&self) -> &String {
        &self.region_name
    }

    pub(crate) fn year(&self) -> i32 {
        self.year
    }

    pub(crate) fn hpi(&self) -> Option<f32> {
        self.hpi
    }

    pub(crate) fn annual_change(&self) -> Option<f32> {
        self.annual_change
    }

    pub(crate) fn hpi_1990_base(&self) -> Option<f32> {
        self.hpi_1990_base
    }

    pub(crate) fn hpi_2000_base(&self) -> Option<f32> {
        self.hpi_2000_base
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HpiData {
    three_zip_hpis: Hpis,
    five_zip_hpis: Hpis,
    county_hpis: Hpis,
}

impl HpiData {
    pub fn three_zip_hpis(&self) -> &Hpis {
        &self.three_zip_hpis
    }

    pub fn five_zip_hpis(&self) -> &Hpis {
        &self.five_zip_hpis
    }

    pub fn county_hpis(&self) -> &Hpis {
        &self.county_hpis
    }
}

pub type Hpis = Vec<Hpi>;

#[cfg(feature = "db")]
#[derive(Clone, Debug, Default)]
pub struct HpiQuery {
    // region_type: RegionType
    region_name: String,
    start_date: i32,
    end_date: i32,
    // annual_change: Option<bool>,
    // hpi_2000_base: Option<bool>,
}

#[cfg(feature = "db")]
impl HpiQuery {
    pub fn new(region_name: String, start_date: i32, end_date: i32) -> Self {
        Self {
            region_name,
            start_date,
            end_date,
        }
    }

    pub(crate) fn region_name(&self) -> &str {
        &self.region_name
    }

    pub(crate) fn start_date(&self) -> i32 {
        self.start_date
    }

    pub(crate) fn end_date(&self) -> i32 {
        self.end_date
    }
}
