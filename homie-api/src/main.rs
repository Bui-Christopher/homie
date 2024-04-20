#![deny(clippy::all)]
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, OnceLock};

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use homie_core::adapter::repository::{Config, Repository};
use homie_core::domain::hpi::HpiData;
use homie_core::domain::t_yield::TYield;
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

    let repo: Repository = Repository::new(config);
    let state = Arc::new(AppState::new(repo));

    let app = Router::new()
        .route("/health", get(health))
        .route("/zhvis", get(read_zhvis))
        .route("/hpis", get(read_hpis))
        .route("/yields", get(read_yields))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    Ok(axum::serve(listener, app).await?)
}

async fn health() -> &'static str {
    "Service is running."
}

async fn read_zhvis() -> impl IntoResponse {
    let dummy = HpiData::default();
    (StatusCode::OK, Json(dummy))
}

async fn read_hpis(_param: Option<Query<HpiParam>>) -> Json<HpiData> {
    Json(HpiData::default())
}

async fn read_yields(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let t_yield = TYield::default();
    t_yield.read(state.repo.session(), "key").unwrap();
    (StatusCode::OK, Json(t_yield))
}

#[derive(Debug)]
pub enum ApiError {
    AuthError { status_code: u16, message: String },
    DbError { status_code: u16, message: String },
    RequestError { status_code: u16, message: String },
}

#[derive(Debug, Deserialize, Default)]
pub struct HpiParam {
    pub region: Option<usize>,
    pub range_time: Option<usize>,
    pub interval_time: Option<usize>,
    // pub annual_change: Option<bool>,
    // pub base_2000: Option<bool>,
}

// pub async fn read_hpis(State(state): State, Json(req): Json<Request>) ->
// RespResult<()> {

// TODO: Delete
// Notes for later:
// let tmp: Regions = regions
//     .counties
//     .into_iter()
//     .filter(|region| region.city() == "IRVINE")
//     .collect();
