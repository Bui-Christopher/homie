#![deny(clippy::all)]
use std::error::Error;
use std::sync::{Arc, OnceLock};

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use error::AppError;
use homie_core::adapter::config::Config;
use homie_core::adapter::repository::Repository;
use homie_core::domain::hpi::{Hpi, Hpis};
use homie_core::domain::t_yield::{TYield, TYields};
use homie_core::domain::zhvi::{Zhvi, Zhvis};

use crate::util::*;

mod error;
mod util;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);

    let repo = Repository::new(config).await?;
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
    let hpis = Hpi::read_by_query(state.session(), &query).await?;
    Ok(Json(hpis))
}

async fn read_tyields(
    State(state): State<Arc<AppState>>,
    Query(param): Query<TYieldParam>,
) -> Result<Json<TYields>, AppError> {
    let query = param.try_into()?;
    let t_yields = TYield::read_by_query(state.session(), &query).await?;
    Ok(Json(t_yields))
}

async fn read_zhvis(
    State(state): State<Arc<AppState>>,
    Query(param): Query<ZhviParam>,
) -> Result<Json<Zhvis>, AppError> {
    let query = param.try_into()?;
    let zhvis = Zhvi::read_by_query(state.session(), &query).await?;
    Ok(Json(zhvis))
}
