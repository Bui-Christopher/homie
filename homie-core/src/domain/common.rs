use serde::{Deserialize, Serialize};

use crate::domain::hpi::HpiData;
use crate::domain::region::RegionData;
use crate::domain::t_yield::TYieldData;
use crate::domain::zhvi::ZHVIData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Datasets {
    pub hpi_data: HpiData,
    pub region_data: RegionData,
    pub t_yield_data: TYieldData,
    pub zhvi_data: ZHVIData,
}

impl Datasets {
    fn new() -> Self {
        Datasets {
            hpi_data: HpiData {
                three_zip_hpis: vec![],
                five_zip_hpis: vec![],
                county_hpis: vec![],
            },
            region_data: RegionData {
                counties: vec![],
                zipcodes: vec![],
            },
            t_yield_data: TYieldData {
                ten_year_yields: vec![],
            },
            zhvi_data: ZHVIData {
                all_homes_zhvis: vec![],
                condo_coops_zhvis: vec![],
                single_family_homes_zhvis: vec![],
            },
        }
    }
}

impl Default for Datasets {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct CsvRecord(pub(super) Vec<String>);
