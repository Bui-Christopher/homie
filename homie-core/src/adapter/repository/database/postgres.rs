use std::env;
use std::error::Error;

use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use sqlx::{query, query_as, Pool, Postgres};

use crate::adapter::repository::{Config, Persist};
use crate::domain::hpi::{Hpi, HpiPersist, HpiQuery, Hpis};
use crate::domain::t_yield::{TYield, TYieldPersist, TYieldQuery, TYields};
use crate::domain::zhvi::{Zhvi, ZhviPersist, ZhviQuery, Zhvis};

pub struct PostgresClient {
    pool: Pool<Postgres>,
}

impl PostgresClient {
    pub async fn new(_config: &Config) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        PostgresClient { pool }
    }

    fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl Persist for PostgresClient {}

#[async_trait]
impl HpiPersist for PostgresClient {
    async fn create_hpi(&self, hpi: &Hpi) -> Result<(String, i32), Box<dyn Error>> {
        let record = query!(
            r#"
            INSERT INTO hpis
            (region, year, hpi, annual_change, hpi_1990_base, hpi_2000_base)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (region, year) DO NOTHING
            RETURNING region, year;
            "#,
            &hpi.region(),
            &hpi.year(),
            hpi.hpi() as Option<f32>,
            hpi.annual_change() as Option<f32>,
            hpi.hpi_1990_base() as Option<f32>,
            hpi.hpi_2000_base() as Option<f32>,
        )
        .fetch_one(self.pool())
        .await?;
        Ok((record.region, record.year))
    }

    async fn read_hpi_by_id(&self, id: (&str, i32)) -> Result<Hpi, Box<dyn Error>> {
        let record = query_as!(
            Hpi,
            r#"SELECT * FROM hpis WHERE region = $1 AND year = $2"#,
            id.0,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(record)
    }

    async fn update_hpi(&self, hpi: &Hpi) -> Result<(), Box<dyn Error>> {
        query!(
            r#"
            UPDATE hpis 
            SET hpi = $3
            WHERE region = $1 AND year = $2
            RETURNING region, year
            "#,
            hpi.region(),
            hpi.year(),
            hpi.hpi() as Option<f32>,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn delete_hpi_by_id(&self, id: (&str, i32)) -> Result<(), Box<dyn Error>> {
        query!(
            r#"
                DELETE FROM hpis 
                WHERE region = $1 AND year = $2
                RETURNING region, year;
            "#,
            id.0,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn read_hpi_by_query(&self, hpi_query: &HpiQuery) -> Result<Hpis, Box<dyn Error>> {
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
            .fetch_all(self.pool())
            .await?;
        // .unwrap_or_else(|_| Vec::new());
        Ok(hpis)
    }
}

#[async_trait]
impl TYieldPersist for PostgresClient {
    async fn create_t_yield(
        &self,
        t_yield: &TYield,
    ) -> Result<(String, NaiveDate), Box<dyn Error>> {
        let record = query!(
            r#"
            INSERT INTO tyields
            (term, date, yield_return)
            VALUES ($1, $2, $3)
            ON CONFLICT (term, date) DO NOTHING
            RETURNING term, date;
            "#,
            t_yield.term(),
            t_yield.date(),
            *t_yield.yield_return() as Option<f32>
        )
        .fetch_one(self.pool())
        .await?;
        Ok((record.term, record.date))
    }

    async fn read_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<TYield, Box<dyn Error>> {
        let record = query_as!(
            TYield,
            r#"
            SELECT * FROM tyields
            WHERE term = $1 AND date = $2"#,
            id.0,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(record)
    }

    async fn update_t_yield(&self, t_yield: &TYield) -> Result<(), Box<dyn Error>> {
        // TODO: Debug. Not sure why but we need RETURNING * here
        query!(
            r#"
            UPDATE tyields
            SET yield_return = $3
            WHERE term = $1 AND date = $2
            RETURNING *;
            "#,
            t_yield.term(),
            t_yield.date(),
            t_yield.yield_return() as _,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn delete_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<(), Box<dyn Error>> {
        query!(
            r#"
            DELETE FROM tyields
            WHERE term = $1 AND date = $2
            "#,
            id.0,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn read_t_yield_by_query(
        &self,
        t_yield_query: &TYieldQuery,
    ) -> Result<TYields, Box<dyn Error>> {
        let query = match t_yield_query.interval_date() {
            "Day" => {
                "SELECT term, CAST(date AS DATE) AS date, CAST(AVG(yield_return) AS FLOAT4) AS \
                 yield_return FROM tyields WHERE date BETWEEN $1 AND $2 GROUP BY term, date ORDER \
                 BY date"
            }
            "Month" => {
                "SELECT term, CAST(DATE_TRUNC('month', date) AS DATE) AS date, \
                 CAST(AVG(yield_return) AS FLOAT4) AS yield_return FROM tyields WHERE date BETWEEN \
                 $1 AND $2 GROUP BY term, DATE_TRUNC('month', date) ORDER BY date"
            }
            "Year" => {
                "SELECT term, CAST(DATE_TRUNC('year', date) AS DATE) AS date, \
                 CAST(AVG(yield_return) AS FLOAT4) AS yield_return FROM tyields WHERE date BETWEEN \
                 $1 AND $2 GROUP BY term, DATE_TRUNC('year', date) ORDER BY date"
            }
            _ => {
                return Err(Box::new(sqlx::Error::Protocol(
                    "Invalid interval_date".into(),
                )))
            }
        };

        let yields: Vec<TYield> = query_as(query)
            .bind(t_yield_query.start_date())
            .bind(t_yield_query.end_date())
            .fetch_all(self.pool())
            .await?;
        // .unwrap_or_else(|_| Vec::new());
        Ok(yields)
    }
}

impl ZhviPersist for PostgresClient {
    fn create_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi create for: {:?} from PostgresClient.", zhvi);
        Ok(true)
    }

    fn read_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_zhvi(&self, zhvi: &Zhvi) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi update for: {:?} from PostgresClient.", zhvi);
        Ok(true)
    }

    fn delete_zhvi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling zhvi delete with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<Zhvis, Box<dyn Error>> {
        println!("Calling zhvi read by: {:?} from PostgresClient.", query);
        Ok(Zhvi::generate_dummy_data())
    }
}
