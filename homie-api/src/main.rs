#![deny(clippy::all)]
use std::sync::{Arc, OnceLock};

use axum::extract::{MatchedPath, Query, Request, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::Form;
use error::AppError;
use homie_core::adapter::config::Config;
use homie_core::adapter::repository::Repository;
use homie_core::domain::hpi::{Hpi, Hpis};
use homie_core::domain::region::{Region, Regions};
use homie_core::domain::t_yield::{TYield, TYields};
use homie_core::domain::zhvi::{Zhvi, Zhvis};
use tower_http::trace::TraceLayer;

use crate::util::*;

mod error;
mod util;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = CONFIG.get_or_init(Config::load_config);
    init_tracing()?;

    let repo = Repository::new(config).await.map_err(|e| {
        tracing::warn!("Failed to set up repository");
        AppError::Fetch(e.to_string())
    })?;
    let state = Arc::new(AppState::new(repo));

    let app = Router::new()
        .route("/health", get(health))
        .route("/hpis", get(read_hpis))
        .route("/regions", post(read_regions))
        .route("/tyields", get(read_tyields))
        .route("/zhvis", get(read_zhvis))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());
                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                .on_failure(()),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> &'static str {
    "Service is running."
}

async fn read_hpis(
    State(state): State<Arc<AppState>>,
    Query(param): Query<HpiParam>,
) -> Result<Json<Hpis>, AppError> {
    tracing::debug!("Reading HPIs with {:?}", serde_json::to_string(&param)?);
    let query = param.try_into()?;
    let hpis = Hpi::read_by_query(state.session(), &query).await?;
    Ok(Json(hpis))
}

async fn read_regions(
    State(state): State<Arc<AppState>>,
    Form(param): Form<RegionParam>,
) -> Result<Json<Regions>, AppError> {
    tracing::debug!("Reading Regions with {:?}", serde_json::to_string(&param)?);
    let query = param.into();
    let regions = Region::read_by_query(state.session(), &query).await?;
    Ok(Json(regions))
}

async fn read_tyields(
    State(state): State<Arc<AppState>>,
    Query(param): Query<TYieldParam>,
) -> Result<Json<TYields>, AppError> {
    tracing::debug!("Reading TYields with {:?}", serde_json::to_string(&param)?);
    let query = param.try_into()?;
    let t_yields = TYield::read_by_query(state.session(), &query).await?;
    Ok(Json(t_yields))
}

async fn read_zhvis(
    State(state): State<Arc<AppState>>,
    Query(param): Query<ZhviParam>,
) -> Result<Json<Zhvis>, AppError> {
    tracing::debug!("Reading Zhvis with {:?}", serde_json::to_string(&param)?);
    let query = param.try_into()?;
    let zhvis = Zhvi::read_by_query(state.session(), &query).await?;
    Ok(Json(zhvis))
}
