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
use homie_core::domain::hpi::{Hpi, HpiQuery};
use homie_core::domain::t_yield::{TYield, TYieldQuery};
use homie_core::domain::zhvi::{Zhvi, ZhviQuery};
use serde::{Deserialize, Serialize};

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
        .route("/yields", get(read_yields))
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
    param: Option<Query<HpiParam>>,
) -> impl IntoResponse {
    let Query(_param) = param.unwrap_or_default();
    let res = Hpi::read_by_query(state.repo.session(), &HpiQuery::default()).unwrap();
    (StatusCode::OK, Json(res))
}

async fn read_yields(
    State(state): State<Arc<AppState>>,
    param: Option<Query<TYieldParam>>,
) -> impl IntoResponse {
    let Query(_param) = param.unwrap_or_default();
    let res = TYield::read_by_query(state.repo.session(), &TYieldQuery::default())
        .await
        .unwrap();
    (StatusCode::OK, Json(res))
}

async fn read_zhvis(
    State(state): State<Arc<AppState>>,
    param: Option<Query<ZhviParam>>,
) -> impl IntoResponse {
    let Query(_param) = param.unwrap_or_default();
    let res = Zhvi::read_by_query(state.repo.session(), &ZhviQuery::default()).unwrap();
    (StatusCode::OK, Json(res))
}

// TODO: Remove Serialize (testing)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HpiParam {
    pub region_type: String, // Prob some enum
    pub region_id: String,
    pub state_date: String,
    pub end_date: String,
    pub interval: String, // Per Year/Month/Day
    pub annual_change: Option<bool>,
    pub base_2000: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TYieldParam {
    pub state_date: String,
    pub end_date: String,
    pub interval: String, // Per Year/Month/Day
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ZhviParam {
    pub state_date: String,
    pub end_date: String,
    pub interval: String,    // Prob some enum
    pub region_type: String, // Prob some enum
    pub region_id: String,
    pub percentile: String, // Prob some enum
    pub home_type: String,  // Prob some enum
}
