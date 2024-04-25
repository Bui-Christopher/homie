use std::error::Error;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{query, query_as, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TYield {
    id: Uuid,
    date: NaiveDate,
    yield_return: Option<f32>,
}

#[derive(Debug)]
pub struct TYieldQuery {
    start_date: NaiveDate,
    end_date: NaiveDate,
    interval_date: String, // Day, Month, Year
}

impl TYieldQuery {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate, interval_date: String) -> Self {
        Self {
            start_date,
            end_date,
            interval_date,
        }
    }
}

impl TYield {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let date = Local::now().date_naive();
        let yield_return = Some(f32::default());
        Self {
            id,
            date,
            yield_return,
        }
    }

    pub fn set_yield(&mut self, new_yield: Option<f32>) {
        self.yield_return = new_yield
    }

    pub fn set_to_uncalling_year(&mut self) {
        self.date = NaiveDate::from_ymd_opt(1999, 1, 12).unwrap()
    }

    pub async fn create(&self, pool: &Pool<Postgres>) -> Result<Uuid, Box<dyn Error>> {
        // TODO: Find a cleaner way to insert yield_return: Option<f32>
        let uuid = match self.yield_return {
            Some(_) => self.create_some_t_yield(pool).await?,
            None => self.create_empty_t_yield(pool).await?,
        };
        Ok(uuid)
    }

    async fn create_some_t_yield(&self, pool: &Pool<Postgres>) -> Result<Uuid, Box<dyn Error>> {
        let record = query!(
            r#"
            INSERT INTO tyields
            (id, date, yield_return)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO NOTHING
            RETURNING id
            "#,
            &self.id,
            &self.date,
            &self.yield_return as _
        )
        .fetch_one(pool)
        .await?;
        Ok(record.id)
    }

    async fn create_empty_t_yield(&self, pool: &Pool<Postgres>) -> Result<Uuid, Box<dyn Error>> {
        let record = query!(
            r#"
            INSERT INTO tyields
            (id, date)
            VALUES ($1, $2)
            ON CONFLICT (id) DO NOTHING
            RETURNING id
            "#,
            &self.id,
            &self.date,
        )
        .fetch_one(pool)
        .await?;
        Ok(record.id)
    }

    pub async fn read_by_id(pool: &Pool<Postgres>, uuid: &Uuid) -> Result<TYield, Box<dyn Error>> {
        let t_yield = query_as!(TYield, r#"SELECT * FROM tyields where id = ($1)"#, uuid)
            .fetch_one(pool)
            .await?;
        Ok(t_yield)
    }

    pub async fn count_t_yields(pool: &Pool<Postgres>) -> Result<usize, Box<dyn Error>> {
        let record = query!(
            r#"
            SELECT COUNT(*) as count FROM tyields
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(record.count.unwrap_or_default() as usize)
    }

    pub async fn update(&self, pool: &Pool<Postgres>) -> Result<Uuid, Box<dyn Error>> {
        let uuid = match self.yield_return {
            Some(_) => self.update_with_yield_return(pool).await?,
            None => self.update_without_yield_return(pool).await?,
        };
        Ok(uuid)
    }

    async fn update_with_yield_return(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<Uuid, Box<dyn Error>> {
        let record = query!(
            r#"
            UPDATE tyields
            SET date = $2, yield_return = $3
            WHERE id = $1
            RETURNING id
            "#,
            &self.id,
            &self.date,
            &self.yield_return as _,
        )
        .fetch_one(pool)
        .await?;
        Ok(record.id)
    }

    async fn update_without_yield_return(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<Uuid, Box<dyn Error>> {
        let record = query!(
            r#"
            UPDATE tyields
            SET date = $2
            WHERE id = $1
            RETURNING id
            "#,
            &self.id,
            &self.date,
        )
        .fetch_one(pool)
        .await?;
        Ok(record.id)
    }

    pub async fn delete_by_id(pool: &Pool<Postgres>, uuid: &Uuid) -> Result<Uuid, Box<dyn Error>> {
        let record = query!(
            r#"
                DELETE FROM tyields
                WHERE id = $1
                RETURNING id;
            "#,
            &uuid,
        )
        .fetch_one(pool)
        .await?;
        Ok(record.id)
    }

    pub async fn read_by_query(
        pool: &sqlx::PgPool,
        t_yield_query: TYieldQuery,
    ) -> Result<Vec<TYield>, sqlx::Error> {
        let query_str = match t_yield_query.interval_date.as_str() {
            "Day" => {
                "SELECT id, CAST(date AS DATE) AS date, CAST(AVG(yield_return) AS FLOAT4) AS \
                 yield_return FROM tyields WHERE date BETWEEN $1 AND $2 GROUP BY id, date ORDER BY \
                 date"
            }
            "Month" => {
                "SELECT id, CAST(DATE_TRUNC('month', date) AS DATE) AS date, \
                 CAST(AVG(yield_return) AS FLOAT4) AS yield_return FROM tyields WHERE date BETWEEN \
                 $1 AND $2 GROUP BY id, DATE_TRUNC('month', date) ORDER BY date"
            }
            "Year" => {
                "SELECT id, CAST(DATE_TRUNC('year', date) AS DATE) AS date, CAST(AVG(yield_return) \
                 AS FLOAT4) AS yield_return FROM tyields WHERE date BETWEEN $1 AND $2 GROUP BY id, \
                 DATE_TRUNC('year', date) ORDER BY date"
            }
            _ => return Err(sqlx::Error::Protocol("Invalid interval_date".into())),
        };

        let yields: Vec<TYield> = sqlx::query_as(query_str)
            .bind(t_yield_query.start_date)
            .bind(t_yield_query.end_date)
            .fetch_all(pool)
            .await?;

        Ok(yields)
    }
}
