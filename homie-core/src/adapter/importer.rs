use crate::adapter::config::Config;
use crate::domain::hpi::{read_fhfa_hpis, HpiConfig, HpiData};
use crate::domain::region::{read_huduser_regions, RegionConfig, RegionData};
use crate::domain::t_yield::{read_fed_yields, TYieldConfig, TYieldData};
use crate::domain::zhvi::{read_zillow_zhvis, ZhviConfig, ZhviData};
use crate::error::DomainError;

pub struct Importer {
    hpi_config: HpiConfig,
    region_config: RegionConfig,
    t_yield_config: TYieldConfig,
    zhvi_config: ZhviConfig,
}

impl Importer {
    pub fn new(config: &'static Config) -> Self {
        let hpi_config = config.hpi_config();
        let region_config = config.region_config();
        let t_yield_config = config.t_yield_config();
        let zhvi_config = config.zhvi_config();

        Importer {
            t_yield_config,
            region_config,
            hpi_config,
            zhvi_config,
        }
    }
}

impl Importer {
    pub fn read_fhfa_hpis(&self) -> Result<HpiData, DomainError> {
        read_fhfa_hpis(self.hpi_config())
    }

    pub fn read_fed_yields(&self) -> Result<TYieldData, DomainError> {
        read_fed_yields(self.t_yield_config())
    }

    pub fn read_huduser_regions(&self) -> Result<RegionData, DomainError> {
        read_huduser_regions(self.region_config())
    }

    pub fn read_zillow_zhvis(&self) -> Result<ZhviData, DomainError> {
        read_zillow_zhvis(self.zhvi_config())
    }

    fn hpi_config(&self) -> &HpiConfig {
        &self.hpi_config
    }

    fn region_config(&self) -> &RegionConfig {
        &self.region_config
    }

    fn t_yield_config(&self) -> &TYieldConfig {
        &self.t_yield_config
    }

    fn zhvi_config(&self) -> &ZhviConfig {
        &self.zhvi_config
    }
}
