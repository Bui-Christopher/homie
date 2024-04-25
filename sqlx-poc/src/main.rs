#![deny(clippy::all)]
use std::error::Error;

use models::TYield;
use sqlx::postgres::PgPoolOptions;

use crate::models::TYieldQuery;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5433/homie")
        .await?;

    // Create
    let t_yield = TYield::default();
    t_yield.create(&pool).await?;

    let t_yield = TYield::new((2001, 1, 1));
    let (term, date) = t_yield.create(&pool).await?;

    // Read
    let mut t_yield = TYield::read_by_id(&pool, (&term, &date)).await?;
    println!("Reading from postgres: {:?}", t_yield);
    println!(
        "There are {} element(s) in the DB",
        TYield::count_t_yields(&pool).await?
    );

    t_yield.set_yield(None);
    t_yield.update(&pool).await?;
    println!("Reading the updated t_yield from postgres: {:?}", t_yield);

    let t_yields = TYield::read_by_query(&pool, TYieldQuery::new()).await?;
    println!("There queried t_yields are: {:?}", t_yields);

    // Delete
    TYield::delete_by_id(&pool, &term, &date).await?;
    println!(
        "There are now {} element(s) in the DB",
        TYield::count_t_yields(&pool).await?
    );

    Ok(())
}
