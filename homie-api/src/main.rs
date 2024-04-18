#![deny(clippy::all)]
use std::error::Error;
use std::sync::OnceLock;

use adapter::repository::database::postgres::PostgresClient;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::adapter::repository::database::http::HttpClient;
use crate::adapter::repository::Repository;
use crate::config::Config;

mod adapter;
mod config;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);
    let client = HttpClient::new();
    let _postgres = PostgresClient::new();
    let repository = Repository::new(config, client);

    // TODO: Remove (testing)
    let datasets = HpiData::default();
    repository.run_all_crud(&datasets)?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/zhvis", get(read_zhvis))
        .route("/hpis/:id", get(read_hpis))
        .route("/yields", get(read_yields));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    Ok(axum::serve(listener, app).await?)
}

async fn health() -> &'static str {
    "Ok"
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Param {
    pub offset: Option<usize>,
    pub limit: Option<String>,
}

// async fn todos_index(
//     pagination: Option<Query<Pagination>>,

async fn read_zhvis() -> impl IntoResponse {
    let dummy = HpiData::default();
    (StatusCode::OK, Json(dummy))
}

// pub async fn read_hpis(State(state): State, Json(req): Json<Request>) ->
// RespResult<()> {
async fn read_hpis(param: Option<Query<Param>>) -> Json<HpiData> {
    let dummy = HpiData {
        region: "".to_string(),
        limit: param.unwrap().0.limit.unwrap().to_owned(),
    };
    Json(dummy)
}

async fn read_yields() -> impl IntoResponse {
    let dummy = HpiData::default();
    (StatusCode::OK, Json(dummy))
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HpiData {
    pub region: String,
    pub limit: String,
}

#[derive(Debug)]
pub enum ApiError {
    AuthError { status_code: u16, message: String },
    DbError { status_code: u16, message: String },
    RequestError { status_code: u16, message: String },
}
// TokenError(#[from] TokenError),
// UserError(#[from] UserError),
// DbError(#[from] DbError),

// #[derive(Debug, Error)]
// pub enum RequestError {
//     #[error(transparent)]
//     ValidationError(#[from] validator::ValidationErrors),
//     #[error(transparent)]
//     JsonRejection(#[from] JsonRejection),
// }
// TODO: Delete
// Notes for later:
// let tmp: Regions = regions
//     .counties
//     .into_iter()
//     .filter(|region| region.city() == "IRVINE")
//     .collect();
