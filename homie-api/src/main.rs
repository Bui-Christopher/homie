#![deny(clippy::all)]
use std::error::Error;
use std::fmt::Debug;
use std::sync::OnceLock;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use homie_core::adapter::repository::database::common::CRUDOperations;
use homie_core::adapter::repository::database::postgres::Postgres;
use homie_core::adapter::repository::{Config, Repository};
use homie_core::domain::common::Datasets;
use homie_core::domain::hpi::HpiData;
use serde::Deserialize;

mod error;

struct AppState<D: CRUDOperations<T>, T: Debug> {
    repo: Repository<D, T>,
}

impl<D: CRUDOperations<T>, T: Debug> AppState<D, T> {
    fn new(repo: Repository<D, T>) -> Self {
        Self { repo }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);

    // TODO: Generate a client by config
    // let repo = Repository::new(config);
    let client = Postgres::new();
    let repo = Repository::new(config, client);
    let state = AppState::new(repo);

    // TODO: Remove (testing)
    let datasets = Datasets::default();
    state.repo.write_data(&datasets)?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/zhvis", get(read_zhvis))
        .route("/hpis", get(read_hpis))
        .route("/yields", get(read_yields));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    Ok(axum::serve(listener, app).await?)
}

async fn health() -> &'static str {
    "Ok"
}

async fn read_zhvis() -> impl IntoResponse {
    let dummy = HpiData::default();
    (StatusCode::OK, Json(dummy))
}

async fn read_hpis(_param: Option<Query<HpiParam>>) -> Json<HpiData> {
    Json(HpiData::default())
}

async fn read_yields() -> impl IntoResponse {
    let dummy = HpiData::default();
    (StatusCode::OK, Json(dummy))
}

#[derive(Debug)]
pub enum ApiError {
    AuthError { status_code: u16, message: String },
    DbError { status_code: u16, message: String },
    RequestError { status_code: u16, message: String },
}

// The query parameters for todos index
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
