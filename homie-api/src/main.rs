#![deny(clippy::all)]
use std::error::Error;
use std::sync::OnceLock;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::adapter::Adapter;
use crate::config::Config;

mod adapter;
mod config;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = CONFIG.get_or_init(Config::load_config);
    let _reader = Adapter::new(config);

    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    Ok(axum::serve(listener, app).await?)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// TODO: Delete
// Notes for later:
// let tmp: Regions = regions
//     .counties
//     .into_iter()
//     .filter(|region| region.city() == "IRVINE")
//     .collect();
