#![deny(clippy::all)]
use std::error::Error;

use chrono::NaiveDate;
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
    let t_yield = TYield::new();
    let uuid = t_yield.create(&pool).await?;

    // Read
    let mut t_yield = TYield::read_by_id(&pool, &uuid).await?;
    println!("Reading from postgres: {:?}", t_yield);
    println!(
        "There are {} element(s) in the DB",
        TYield::count_t_yields(&pool).await?
    );

    t_yield.set_yield(None);
    t_yield.set_to_uncalling_year();
    t_yield.update(&pool).await?;
    println!("Reading the updated t_yield from postgres: {:?}", t_yield);

    let start_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let interval_date = "Month".to_string();
    let t_yield_query = TYieldQuery::new(start_date, end_date, interval_date);

    let t_yields = TYield::read_by_query(&pool, t_yield_query).await?;
    println!("There queried t_yields are: {:?}", t_yields);

    // Delete
    // TYield::delete_by_id(&pool, &uuid).await?;
    // println!(
    //     "There are now {} element(s) in the DB",
    //     TYield::count_t_yields(&pool).await?
    // );

    Ok(())
}
