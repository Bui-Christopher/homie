#![deny(clippy::all)]
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use chrono::{Datelike, NaiveDate};
use homie_core::adapter::repository::{Config, Repository};
use homie_core::domain::hpi::{Hpi, HpiQuery};
use homie_core::domain::t_yield::{TYield, TYieldQuery};
use homie_core::domain::zhvi::{Zhvi, ZhviQuery};
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
) -> impl IntoResponse {
    let query = param.into();
    let res = Hpi::read_by_query(state.repo.session(), &query)
        .await
        .unwrap();
    (StatusCode::OK, Json(res))
}

async fn read_tyields(
    State(state): State<Arc<AppState>>,
    Query(param): Query<TYieldParam>,
) -> impl IntoResponse {
    let query = param.into();
    let res = TYield::read_by_query(state.repo.session(), &query)
        .await
        .unwrap();
    (StatusCode::OK, Json(res))
}

async fn read_zhvis(
    State(state): State<Arc<AppState>>,
    Query(param): Query<ZhviParam>,
) -> impl IntoResponse {
    let query = param.into();
    let res = Zhvi::read_by_query(state.repo.session(), &query)
        .await
        .unwrap();
    (StatusCode::OK, Json(res))
}

#[derive(Debug, Default, Deserialize)]
pub struct HpiParam {
    pub start_date: String,
    pub end_date: String,
    pub interval_date: Option<String>,
    pub region_type: Option<String>,
    pub region_name: String,
    pub annual_change: Option<bool>,
    pub base_2000: Option<bool>,
}

impl From<HpiParam> for HpiQuery {
    fn from(value: HpiParam) -> Self {
        let start_date = NaiveDate::parse_from_str(&value.start_date, "%Y-%m-%d").unwrap();
        let end_date = NaiveDate::parse_from_str(&value.end_date, "%Y-%m-%d").unwrap();
        let region_name = value.region_name.clone();
        HpiQuery::new(region_name, start_date.year(), end_date.year())
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct TYieldParam {
    pub start_date: String,
    pub end_date: String,
    pub interval_date: String, // Day, Month, Year
}

impl From<TYieldParam> for TYieldQuery {
    fn from(value: TYieldParam) -> Self {
        let start_date = NaiveDate::parse_from_str(&value.start_date, "%Y-%m-%d").unwrap();
        let end_date = NaiveDate::parse_from_str(&value.end_date, "%Y-%m-%d").unwrap();
        let interval_date = value.interval_date.clone(); // Day, Month, Year
        TYieldQuery::new(start_date, end_date, interval_date)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct ZhviParam {
    pub start_date: String,
    pub end_date: String,
    pub interval_date: String,
    pub home_type: String,
    pub region_type: String,
    pub region_name: String,
    pub percentile: String,
}

impl From<ZhviParam> for ZhviQuery {
    fn from(value: ZhviParam) -> Self {
        let start_date = NaiveDate::parse_from_str(&value.start_date, "%Y-%m-%d").unwrap();
        let end_date = NaiveDate::parse_from_str(&value.end_date, "%Y-%m-%d").unwrap();
        let interval_date = value.interval_date.clone(); // Day, Month, Year
        let home_type = value.home_type.clone();
        let region_type = value.region_type.clone();
        let region_name = value.region_name.clone();
        let percentile = value.percentile.clone();
        Self::new(
            start_date,
            end_date,
            interval_date,
            home_type,
            region_type,
            region_name,
            percentile,
        )
    }
}
