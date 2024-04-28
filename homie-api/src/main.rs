#![deny(clippy::all)]
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{Datelike, NaiveDate};
use error::AppError;
use homie_core::adapter::repository::{Config, Repository};
use homie_core::domain::hpi::{Hpi, HpiQuery, Hpis};
use homie_core::domain::t_yield::{TYield, TYieldQuery, TYields};
use homie_core::domain::zhvi::{Zhvi, ZhviQuery, Zhvis};
use serde::Deserialize;

mod error;

struct AppState {
    repo: Repository,
}

impl AppState {
    fn new(repo: Repository) -> Self {
        Self { repo }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);

    let repo: Repository = Repository::new(config).await;
    let state = Arc::new(AppState::new(repo));

    let app = Router::new()
        .route("/health", get(health))
        .route("/hpis", get(read_hpis))
        // .route("/regions", get(read_regions))
        .route("/tyields", get(read_tyields))
        .route("/zhvis", get(read_zhvis))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    Ok(axum::serve(listener, app).await?)
}

async fn health() -> &'static str {
    "Service is running."
}

async fn read_hpis(
    State(state): State<Arc<AppState>>,
    Query(param): Query<HpiParam>,
) -> Result<Json<Hpis>, AppError> {
    let query = param.try_into()?;
    let res = Hpi::read_by_query(state.repo.session(), &query).await?;
    Ok(Json(res))
}

async fn read_tyields(
    State(state): State<Arc<AppState>>,
    Query(param): Query<TYieldParam>,
) -> Result<Json<TYields>, AppError> {
    let query = param.try_into()?;
    let res = TYield::read_by_query(state.repo.session(), &query).await?;
    Ok(Json(res))
}

async fn read_zhvis(
    State(state): State<Arc<AppState>>,
    Query(param): Query<ZhviParam>,
) -> Result<Json<Zhvis>, AppError> {
    let query = param.try_into()?;
    let res = Zhvi::read_by_query(state.repo.session(), &query).await?;
    Ok(Json(res))
}

#[derive(Debug, Default, Deserialize)]
struct HpiParam {
    region_name: String,
    start_date: String,
    end_date: String,
    // interval_date: String,
    // region_type: String,
    // annual_change: bool,
    // base_2000: bool,
}

impl TryFrom<HpiParam> for HpiQuery {
    type Error = AppError;

    fn try_from(value: HpiParam) -> Result<Self, Self::Error> {
        let region_name = value.region_name.clone();
        let start_date = parse_naive_date(&value.start_date)?;
        let end_date = parse_naive_date(&value.end_date)?;
        Ok(HpiQuery::new(
            region_name,
            start_date.year(),
            end_date.year(),
        ))
    }
}

#[derive(Debug, Default, Deserialize)]
struct TYieldParam {
    start_date: String,
    end_date: String,
    interval_date: String, // Day, Month, Year
}

impl TryFrom<TYieldParam> for TYieldQuery {
    type Error = AppError;

    fn try_from(value: TYieldParam) -> Result<Self, Self::Error> {
        let start_date = parse_naive_date(&value.start_date)?;
        let end_date = parse_naive_date(&value.end_date)?;
        let interval_date = value.interval_date.clone(); // Day, Month, Year
        Ok(TYieldQuery::new(start_date, end_date, interval_date))
    }
}

#[derive(Debug, Deserialize)]
struct ZhviParam {
    start_date: String,
    end_date: String,
    interval_date: String,
    home_type: String,
    region_type: String,
    region_name: String,
    percentile: String,
}

impl TryFrom<ZhviParam> for ZhviQuery {
    type Error = AppError;

    fn try_from(value: ZhviParam) -> Result<Self, Self::Error> {
        let start_date = parse_naive_date(&value.start_date)?;
        let end_date = parse_naive_date(&value.end_date)?;
        let interval_date = value.interval_date.clone(); // Day, Month, Year
        let home_type = value.home_type.clone();
        let region_type = value.region_type.clone();
        let region_name = value.region_name.clone();
        let percentile = value.percentile.clone();
        Ok(Self::new(
            start_date,
            end_date,
            interval_date,
            home_type,
            region_type,
            region_name,
            percentile,
        ))
    }
}

fn parse_naive_date(input: &str) -> Result<NaiveDate, AppError> {
    Ok(NaiveDate::parse_from_str(input, "%Y-%m-%d")?)
}
