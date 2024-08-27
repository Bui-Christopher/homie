use std::env;

use crate::domain::hpi::HpiConfig;
use crate::domain::region::RegionConfig;
use crate::domain::t_yield::TYieldConfig;
use crate::domain::zhvi::ZhviConfig;

pub struct Config {
    use_zillow_api: bool,
    hpi_config: HpiConfig,
    region_config: RegionConfig,
    t_yield_config: TYieldConfig,
    zhvi_config: ZhviConfig,
}

impl Config {
    pub fn load_config() -> Config {
        let ten_year_yield_path = env::var("TEN_YEAR_YIELD_PATH").ok();
        let t_yield_config = TYieldConfig::new(ten_year_yield_path);

        let three_zip_hpis_path = env::var("THREE_ZIP_HPIS_PATH").ok();
        let five_zip_hpis_path = env::var("FIVE_ZIP_HPIS_PATH").ok();
        let county_hpis_path = env::var("COUNTY_HPIS_PATH").ok();
        let hpi_config = HpiConfig::new(three_zip_hpis_path, five_zip_hpis_path, county_hpis_path);

        let cities_path = env::var("CITIES_PATH").ok();
        let zip_county_path = env::var("ZIP_COUNTY_PATH").ok();
        let region_config = RegionConfig::new(cities_path, zip_county_path);

        let mid_zip_all_homes_path = env::var("MID_ZIP_ALL_HOMES_PATH").ok();
        let mid_city_all_homes_path = env::var("MID_CITY_ALL_HOMES_PATH").ok();
        let mid_county_all_homes_path = env::var("MID_COUNTY_ALL_HOMES_PATH").ok();
        let zhvi_config = ZhviConfig::new(
            mid_zip_all_homes_path,
            mid_city_all_homes_path,
            mid_county_all_homes_path,
        );

        Config {
            hpi_config,
            use_zillow_api: false,
            region_config,
            t_yield_config,
            zhvi_config,
        }
    }

    pub fn is_zillow_api_enabled(&self) -> bool {
        self.use_zillow_api
    }

    pub(crate) fn hpi_config(&self) -> HpiConfig {
        self.hpi_config.clone()
    }

    pub(crate) fn region_config(&self) -> RegionConfig {
        self.region_config.clone()
    }

    pub(crate) fn t_yield_config(&self) -> TYieldConfig {
        self.t_yield_config.clone()
    }

    pub(crate) fn zhvi_config(&self) -> ZhviConfig {
        self.zhvi_config.clone()
    }
}
