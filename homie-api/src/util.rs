use std::fmt::Debug;

use chrono::{Datelike, NaiveDate};
use homie_core::adapter::repository::{Persist, Repository};
use homie_core::domain::common::{DateInterval, RegionType};
use homie_core::domain::hpi::HpiQuery;
use homie_core::domain::region::RegionQuery;
use homie_core::domain::t_yield::TYieldQuery;
use homie_core::domain::zhvi::{HomeType, Percentile, ZhviQuery};
use serde::{Deserialize, Serialize};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::IntoParams;

use crate::error::AppError;

pub(crate) struct AppState {
    repo: Repository,
}

impl AppState {
    pub(crate) fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub(crate) fn session(&self) -> &dyn Persist {
        self.repo.session()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct HpiParam {
    // region_type: String,
    region_name: String,
    start_date: String,
    end_date: String,
    // annual_change: bool,
    // base_2000: bool,
}

impl TryFrom<HpiParam> for HpiQuery {
    type Error = AppError;

    fn try_from(param: HpiParam) -> Result<Self, Self::Error> {
        let region_name = param.region_name.clone();
        let start_date = parse_naive_date(&param.start_date)?;
        let end_date = parse_naive_date(&param.end_date)?;
        Ok(HpiQuery::new(
            region_name,
            start_date.year(),
            end_date.year(),
        ))
    }
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub(crate) struct RegionParam {
    #[serde(default)]
    cities: Vec<String>,
    #[serde(default)]
    zipcodes: Vec<String>,
}

impl From<RegionParam> for RegionQuery {
    fn from(param: RegionParam) -> Self {
        let cities = param
            .cities
            .into_iter()
            .map(|mut city| {
                city.make_ascii_lowercase();
                city
            })
            .collect();
        let zipcodes = param.zipcodes;
        RegionQuery::new(cities, zipcodes)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct TYieldParam {
    start_date: String,
    end_date: String,
    date_interval: String,
}

impl TryFrom<TYieldParam> for TYieldQuery {
    type Error = AppError;

    fn try_from(param: TYieldParam) -> Result<Self, Self::Error> {
        let start_date = parse_naive_date(&param.start_date)?;
        let end_date = parse_naive_date(&param.end_date)?;
        let date_interval = parse_date_interval(&param.date_interval)?;
        Ok(TYieldQuery::new(start_date, end_date, date_interval))
    }
}

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub(crate) struct ZhviParam {
    start_date: String,
    end_date: String,
    date_interval: String,
    home_type: String,
    region_type: String,
    region_name: String,
    percentile: String,
}

impl TryFrom<ZhviParam> for ZhviQuery {
    type Error = AppError;

    fn try_from(param: ZhviParam) -> Result<Self, Self::Error> {
        let start_date = parse_naive_date(&param.start_date)?;
        let end_date = parse_naive_date(&param.end_date)?;
        let date_interval = parse_date_interval(&param.date_interval)?;
        let region_name = param.region_name.clone();
        let region_type = parse_region_type(&param.region_type)?;
        let home_type = parse_home_type(&param.home_type)?;
        let percentile = parse_percentile(&param.percentile)?;
        Ok(Self::new(
            start_date,
            end_date,
            date_interval,
            region_name,
            region_type,
            home_type,
            percentile,
        ))
    }
}

fn parse_home_type(input: &str) -> Result<HomeType, AppError> {
    HomeType::try_from(input.to_ascii_lowercase().as_str())
        .map_err(|_| AppError::Request("Failed to read home type".to_string()))
}

fn parse_date_interval(input: &str) -> Result<DateInterval, AppError> {
    DateInterval::try_from(input.to_ascii_lowercase().as_str())
        .map_err(|_| AppError::Request("Failed to read date interval".to_string()))
}

fn parse_naive_date(input: &str) -> Result<NaiveDate, AppError> {
    Ok(NaiveDate::parse_from_str(input, "%Y-%m-%d")?)
}

fn parse_percentile(input: &str) -> Result<Percentile, AppError> {
    Percentile::try_from(input.to_ascii_lowercase().as_str())
        .map_err(|_| AppError::Request("Failed to read percentile".to_string()))
}

fn parse_region_type(input: &str) -> Result<RegionType, AppError> {
    RegionType::try_from(input.to_ascii_lowercase().as_str())
        .map_err(|_| AppError::Request("Failed to read region type".to_string()))
}

pub(crate) fn init_tracing() -> Result<(), AppError> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "homie_api=warn".into())
                .add_directive("sqlx::query=warn".parse()?), // Silence SQLx calls
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    Ok(())
}
