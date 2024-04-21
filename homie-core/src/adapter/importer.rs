use std::env;
use std::error::Error;

use crate::adapter::repository::Config;
use crate::domain::hpi::{read_fhfa_hpis, HpiConfig, HpiData};
use crate::domain::t_yield::{read_fed_yields, TYieldConfig, TYieldData};
use crate::domain::zhvi::{read_zillow_zhvis, ZhviConfig, ZhviData};

pub struct Importer {
    t_yield_config: TYieldConfig,
    hpi_config: HpiConfig,
    zhvi_config: ZhviConfig,
    // region_config: RegionConfig,
}

impl Importer {
    pub fn new(_config: &'static Config) -> Self {
        // TODO: Refactor reading from env with Config struct
        // let t_yield_config = config.t_yield_config();
        // let hpi_config = config.hpi_config();
        // let zhvi_config = config.zhvi_config();

        let ten_year_yield_path = env::var("TEN_YEAR_YIELD_PATH").ok();
        let t_yield_config = TYieldConfig::new(ten_year_yield_path);

        let three_zip_hpis_path = env::var("THREE_ZIP_HPIS_PATH").ok();
        let five_zip_hpis_path = env::var("FIVE_ZIP_HPIS_PATH").ok();
        let county_hpis_path = env::var("COUNTY_HPIS_PATH").ok();
        let hpi_config = HpiConfig::new(three_zip_hpis_path, five_zip_hpis_path, county_hpis_path);

        let mid_zip_all_homes_path = env::var("MID_ZIP_ALL_HOMES_PATH").ok();
        let mid_city_all_homes_path = env::var("MID_CITY_ALL_HOMES_PATH").ok();
        let mid_county_all_homes_path = env::var("MID_COUNTY_ALL_HOMES_PATH").ok();
        let zhvi_config = ZhviConfig::new(
            mid_zip_all_homes_path,
            mid_city_all_homes_path,
            mid_county_all_homes_path,
        );

        Importer {
            t_yield_config,
            hpi_config,
            zhvi_config,
        }
    }
}

impl Importer {
    pub fn read_fed_yields(&self) -> Result<TYieldData, Box<dyn Error>> {
        read_fed_yields(self.t_yield_config())
    }

    pub fn read_fhfa_hpis(&self) -> Result<HpiData, Box<dyn Error>> {
        read_fhfa_hpis(self.hpi_config())
    }

    pub fn read_zillow_zhvis(&self) -> Result<ZhviData, Box<dyn Error>> {
        read_zillow_zhvis(self.zhvi_config())
    }

    fn t_yield_config(&self) -> &TYieldConfig {
        &self.t_yield_config
    }

    fn hpi_config(&self) -> &HpiConfig {
        &self.hpi_config
    }

    fn zhvi_config(&self) -> &ZhviConfig {
        &self.zhvi_config
    }
}
