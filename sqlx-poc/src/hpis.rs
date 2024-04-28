use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{query, query_as, Pool, Postgres};

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Hpi {
    region: String,
    year: i32,
    hpi: Option<f32>,
    annual_change: Option<f32>,
    hpi_1990_base: Option<f32>, // TODO: Remove, idc 1990
    hpi_2000_base: Option<f32>,
}

#[derive(Debug)]
pub struct HpiQuery {
    region: String, // ThreeZip, FiveZip, County
    start_date: i32,
    end_date: i32,
}

impl HpiQuery {
    pub fn new() -> Self {
        let region = "Orange".to_string();
        let start_date = 2000;
        let end_date = 2024;
        Self {
            region,
            start_date,
            end_date,
        }
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn start_date(&self) -> i32 {
        self.start_date
    }

    pub fn end_date(&self) -> i32 {
        self.end_date
    }
}

impl Hpi {
    pub fn new(year: i32) -> Self {
        let region = "Orange".to_string();
        let hpi = Some(f32::default());
        let annual_change = None;
        let hpi_1990_base = Some(f32::default());
        let hpi_2000_base = Some(f32::default());
        Self {
            region,
            year,
            hpi,
            annual_change,
            hpi_1990_base,
            hpi_2000_base,
        }
    }

    // pub fn default() -> Self {
    //     Self::new(2024)
    // }

    pub fn set_hpi(&mut self, hpi: Option<f32>) {
        self.hpi = hpi
    }

    pub async fn create(&self, pool: &Pool<Postgres>) -> Result<(String, i32), Box<dyn Error>> {
        let record = query!(
            r#"
            INSERT INTO hpis
            (region, year, hpi, annual_change, hpi_1990_base, hpi_2000_base)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (region, year) DO NOTHING
            RETURNING region, year;
            "#,
            &self.region,
            &self.year,
            self.hpi as Option<f32>,
            self.annual_change as Option<f32>,
            self.hpi_1990_base as Option<f32>,
            self.hpi_2000_base as Option<f32>,
        )
        .fetch_one(pool)
        .await?;
        Ok((record.region, record.year))
    }

    pub async fn read_by_id(pool: &Pool<Postgres>, id: (&str, i32)) -> Result<Hpi, Box<dyn Error>> {
        let record = query_as!(
            Hpi,
            r#"SELECT * FROM hpis WHERE region = $1 AND year = $2"#,
            id.0,
            id.1,
        )
        .fetch_one(pool)
        .await?;
        Ok(record)
    }

    pub async fn update(&self, pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
        query!(
            r#"
            UPDATE hpis 
            SET hpi = $3
            WHERE region = $1 AND year = $2
            RETURNING region, year
            "#,
            &self.region,
            &self.year,
            self.hpi as Option<f32>,
        )
        .fetch_one(pool)
        .await?;
        Ok(())
    }

    pub async fn delete_by_id(
        pool: &Pool<Postgres>,
        region: &str,
        year: i32,
    ) -> Result<(), Box<dyn Error>> {
        query!(
            r#"
                DELETE FROM hpis 
                WHERE region = $1 AND year = $2
                RETURNING region, year;
            "#,
            &region,
            &year,
        )
        .fetch_one(pool)
        .await?;
        Ok(())
    }

    pub async fn read_by_query(
        pool: &sqlx::PgPool,
        hpi_query: HpiQuery,
    ) -> Result<Vec<Hpi>, sqlx::Error> {
        let query = r#"
            SELECT * FROM hpis
            WHERE region = $1
            AND year >= $2
            AND year <= $3
        "#;
        let hpis: Vec<Hpi> = query_as(query)
            .bind(hpi_query.region())
            .bind(hpi_query.start_date())
            .bind(hpi_query.end_date())
            .fetch_all(pool)
            .await?;
        // .unwrap_or_else(|_| Vec::new());
        Ok(hpis)
    }
}
pub async fn test_hpis(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    let hpi = Hpi::new(2015);
    hpi.create(pool).await?;
    let mut hpi = Hpi::read_by_id(pool, ("Orange", 2015)).await?;
    println!("{:?}", hpi);
    hpi.set_hpi(Some(10.0));
    hpi.update(pool).await?;
    let hpi = Hpi::read_by_id(pool, ("Orange", 2015)).await?;
    println!("{:?}", hpi);
    let hpi_query = HpiQuery::new();
    let hpis = Hpi::read_by_query(pool, hpi_query).await?;
    println!("{:?}", hpis);
    Hpi::delete_by_id(pool, "Orange", 2015).await?;

    Ok(())
}
