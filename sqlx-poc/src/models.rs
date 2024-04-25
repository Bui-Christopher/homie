use std::error::Error;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{query, query_as, Pool, Postgres};

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct TYield {
    term: String,
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
    pub fn new() -> Self {
        let start_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let interval_date = "Month".to_string();
        Self {
            start_date,
            end_date,
            interval_date,
        }
    }
}

impl TYield {
    pub fn new(date: (i32, u32, u32)) -> Self {
        let term = "TenYear".to_string();
        let date = NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap();
        let yield_return = Some(f32::default());
        Self {
            term,
            date,
            yield_return,
        }
    }

    pub fn default() -> Self {
        let term = "TenYear".to_string();
        let date = Local::now().date_naive();
        let yield_return = Some(f32::default());
        Self {
            term,
            date,
            yield_return,
        }
    }

    pub fn set_yield(&mut self, new_yield: Option<f32>) {
        self.yield_return = new_yield
    }

    pub async fn create(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<(String, NaiveDate), Box<dyn Error>> {
        // TODO: Find a cleaner way to insert yield_return: Option<f32>
        let (term, date) = match self.yield_return {
            Some(_) => self.create_some_t_yield(pool).await?,
            None => self.create_empty_t_yield(pool).await?,
        };
        Ok((term, date))
    }

    async fn create_some_t_yield(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<(String, NaiveDate), Box<dyn Error>> {
        let record = query!(
            r#"
            INSERT INTO tyields
            (term, date, yield_return)
            VALUES ($1, $2, $3)
            ON CONFLICT (term, date) DO NOTHING
            RETURNING term, date;
            "#,
            &self.term,
            &self.date,
            &self.yield_return as _
        )
        .fetch_one(pool)
        .await?;
        Ok((record.term, record.date))
    }

    async fn create_empty_t_yield(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<(String, NaiveDate), Box<dyn Error>> {
        let record = query!(
            r#"
            INSERT INTO tyields
            (term, date)
            VALUES ($1, $2)
            ON CONFLICT (term, date) DO NOTHING
            RETURNING term, date
            "#,
            &self.term,
            &self.date,
        )
        .fetch_one(pool)
        .await?;
        Ok((record.term, record.date))
    }

    pub async fn read_by_id(
        pool: &Pool<Postgres>,
        id: (&str, &NaiveDate),
    ) -> Result<TYield, Box<dyn Error>> {
        let record = query_as!(
            TYield,
            r#"SELECT * FROM tyields WHERE term = $1 AND date = $2"#,
            id.0,
            id.1,
        )
        .fetch_one(pool)
        .await?;
        Ok(record)
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

    pub async fn update(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<(String, NaiveDate), Box<dyn Error>> {
        let record = query!(
            r#"
            UPDATE tyields
            SET yield_return = $3
            WHERE term = $1 AND date = $2
            RETURNING term, date
            "#,
            &self.term,
            &self.date,
            &self.yield_return as _,
        )
        .fetch_one(pool)
        .await?;
        Ok((record.term, record.date))
    }

    pub async fn delete_by_id(
        pool: &Pool<Postgres>,
        term: &str,
        date: &NaiveDate,
    ) -> Result<(String, NaiveDate), Box<dyn Error>> {
        let record = query!(
            r#"
                DELETE FROM tyields
                WHERE term = $1 AND date = $2
                RETURNING term, date;
            "#,
            &term,
            &date,
        )
        .fetch_one(pool)
        .await?;
        Ok((record.term, record.date))
    }

    pub async fn read_by_query(
        pool: &sqlx::PgPool,
        t_yield_query: TYieldQuery,
    ) -> Result<Vec<TYield>, sqlx::Error> {
        let query = match t_yield_query.interval_date.as_str() {
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
            _ => return Err(sqlx::Error::Protocol("Invalid interval_date".into())),
        };

        let yields: Vec<TYield> = query_as(query)
            .bind(t_yield_query.start_date)
            .bind(t_yield_query.end_date)
            .fetch_all(pool)
            .await?;
        // .unwrap_or_else(|_| Vec::new());
        Ok(yields)
    }
}
