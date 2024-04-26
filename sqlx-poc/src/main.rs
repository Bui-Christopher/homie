#![deny(clippy::all)]
use std::error::Error;

use sqlx::postgres::PgPoolOptions;

mod hpis;
mod tyields;
mod zhvis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5433/homie")
        .await?;

    tyields::test_t_yields(&pool).await?;
    hpis::test_hpis(&pool).await?;
    Ok(())
}
