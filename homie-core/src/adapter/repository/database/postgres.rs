use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use sqlx::{query, query_as, FromRow, Pool, Postgres};

use crate::adapter::repository::{Config, Persist};
use crate::domain::common::{DateInterval, RegionType};
use crate::domain::hpi::*;
use crate::domain::region::{Region, RegionPersist, RegionQuery, Regions, Zipcode};
use crate::domain::t_yield::*;
use crate::domain::zhvi::*;
use crate::error::Error;

pub struct PostgresClient {
    pool: Pool<Postgres>,
}

impl PostgresClient {
    pub async fn new(_config: &Config) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL")?)
            .await?;
        Ok(PostgresClient { pool })
    }

    fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl Persist for PostgresClient {}

#[async_trait]
impl HpiPersist for PostgresClient {
    async fn create_hpi(&self, hpi: &Hpi) -> Result<(String, i32), Error> {
        let record = query!(
            r#"
                INSERT INTO hpis
                (region_name, region_type, year, hpi, annual_change, hpi_1990_base, hpi_2000_base)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (region_name, year) DO NOTHING
                RETURNING region_name, year;
            "#,
            &hpi.region_name(),
            &hpi.region_type() as _,
            &hpi.year(),
            hpi.hpi() as Option<f32>,
            hpi.annual_change() as Option<f32>,
            hpi.hpi_1990_base() as Option<f32>,
            hpi.hpi_2000_base() as Option<f32>,
        )
        .fetch_one(self.pool())
        .await?;
        Ok((record.region_name, record.year))
    }

    async fn read_hpi_by_id(&self, id: (&str, i32)) -> Result<Hpi, Error> {
        let record = query_as!(
            Hpi,
            r#"
                SELECT region_name, region_type AS "region_type: RegionType", year, hpi, annual_change, hpi_1990_base, hpi_2000_base
                FROM hpis
                WHERE region_name = $1 AND year = $2
            "#,
            id.0,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(record)
    }

    async fn update_hpi(&self, hpi: &Hpi) -> Result<(), Error> {
        query!(
            r#"
                UPDATE hpis
                SET hpi = $1
                WHERE region_name = $2 AND year = $3
                RETURNING region_name, year
            "#,
            hpi.hpi() as Option<f32>,
            hpi.region_name(),
            hpi.year(),
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn delete_hpi_by_id(&self, id: (&str, i32)) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM hpis
                WHERE region_name = $1 AND year = $2
                RETURNING region_name, year;
            "#,
            id.0,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn read_hpi_by_query(&self, hpi_query: &HpiQuery) -> Result<Hpis, Error> {
        let query = r#"
            SELECT * FROM hpis
            WHERE region_name = $1
            AND year >= $2
            AND year <= $3
        "#;
        let hpis: Vec<Hpi> = query_as(query)
            .bind(hpi_query.region_name())
            .bind(hpi_query.start_date())
            .bind(hpi_query.end_date())
            .fetch_all(self.pool())
            .await?;
        Ok(hpis)
    }
}

#[async_trait]
impl RegionPersist for PostgresClient {
    async fn create_region(&self, region: &Region) -> Result<Zipcode, Error> {
        let record = query!(
            r#"
                INSERT INTO regions
                (city, zipcode)
                VALUES ($1, $2)
                ON CONFLICT (zipcode) DO UPDATE
                SET city = $1
                RETURNING zipcode;
            "#,
            region.city(),
            region.zipcode(),
        )
        .fetch_one(self.pool())
        .await?;
        Ok(record.zipcode.to_string())
    }

    async fn read_region_by_id(&self, id: &str) -> Result<Region, Error> {
        let record = query_as!(
            Region,
            r#"
                SELECT city, zipcode
                FROM regions
                WHERE zipcode = $1
            "#,
            id,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(record)
    }

    async fn read_regions_by_city(&self, id: &str) -> Result<Regions, Error> {
        let query = r#"
            SELECT * FROM regions
            WHERE city = $1
        "#;
        let regions = query_as(query).bind(id).fetch_all(self.pool()).await?;
        Ok(regions)
    }

    async fn read_regions_by_query(&self, region_query: &RegionQuery) -> Result<Regions, Error> {
        let mut query = "SELECT * FROM regions".to_string();
        let mut params = Vec::<&str>::new();
        let mut where_clause = String::new();

        if !region_query.cities().is_empty() {
            let mut city_placeholders = String::new();
            for (index, city) in region_query.cities().iter().enumerate() {
                if index != 0 {
                    city_placeholders.push_str(", ");
                }
                city_placeholders.push_str(&format!("${}", params.len() + 1));
                params.push(city);
            }
            where_clause.push_str(&format!(" city = {}", city_placeholders));
        }

        if !region_query.zipcodes().is_empty() {
            if !where_clause.is_empty() {
                where_clause.push_str(" OR");
            }
            let mut zipcodes_placeholders = String::new();
            for (index, zipcode) in region_query.zipcodes().iter().enumerate() {
                if index != 0 {
                    zipcodes_placeholders.push_str(", ");
                }
                zipcodes_placeholders.push_str(&format!("${}", params.len() + 1));
                params.push(zipcode);
            }
            where_clause.push_str(&format!(" zipcode = {}", zipcodes_placeholders));
        }

        if !where_clause.is_empty() {
            query.push_str(&format!(" WHERE{}", where_clause));
        }
        println!("{query}");
        let mut query = query_as(&query);
        for i in 0..region_query.cities().len() {
            query = query.bind(region_query.cities()[i].clone());
        }
        for i in 0..region_query.zipcodes().len() {
            query = query.bind(region_query.zipcodes()[i].clone());
        }
        let regions = query.fetch_all(self.pool()).await?;

        Ok(regions)
    }

    async fn delete_region_by_id(&self, id: &str) -> Result<Zipcode, Error> {
        println!(
            "Calling region delete with id: {:?} from PostgresClient.",
            id
        );
        Ok(Zipcode::default())
    }
}

#[async_trait]
impl TYieldPersist for PostgresClient {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<(String, NaiveDate), Error> {
        let record = query!(
            r#"
                INSERT INTO tyields
                (term, date, yield_return)
                VALUES ($1, $2, $3)
                ON CONFLICT (term, date) DO NOTHING
                RETURNING term AS "term: Term", date;
            "#,
            t_yield.term() as _,
            t_yield.date(),
            *t_yield.yield_return() as Option<f32>
        )
        .fetch_one(self.pool())
        .await?;
        Ok((record.term.to_string(), record.date))
    }

    async fn read_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<TYield, Error> {
        let record = query_as!(
            TYield,
            r#"
                SELECT term AS "term: Term", date, yield_return
                FROM tyields
                WHERE term = $1 AND date = $2
            "#,
            id.0 as _,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(record)
    }

    async fn update_t_yield(&self, t_yield: &TYield) -> Result<(), Error> {
        query!(
            r#"
                UPDATE tyields
                SET yield_return = $1
                WHERE term = $2 AND date = $3
            "#,
            t_yield.yield_return() as _,
            t_yield.term() as _,
            t_yield.date(),
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn delete_t_yield_by_id(&self, id: (&str, &NaiveDate)) -> Result<(), Error> {
        query!(
            r#"
                DELETE FROM tyields
                WHERE term = $1 AND date = $2
            "#,
            id.0 as _,
            id.1,
        )
        .fetch_one(self.pool())
        .await?;
        Ok(())
    }

    async fn read_t_yields_by_query(&self, t_yield_query: &TYieldQuery) -> Result<TYields, Error> {
        let query = match t_yield_query.date_interval() {
            DateInterval::Day => {
                "SELECT term, CAST(date AS DATE) AS date, CAST(AVG(yield_return) AS FLOAT4) AS \
                 yield_return FROM tyields WHERE date BETWEEN $1 AND $2 GROUP BY term, date ORDER \
                 BY date"
            }
            DateInterval::Month => {
                "SELECT term, CAST(DATE_TRUNC('month', date) AS DATE) AS date, \
                 CAST(AVG(yield_return) AS FLOAT4) AS yield_return FROM tyields WHERE date BETWEEN \
                 $1 AND $2 GROUP BY term, DATE_TRUNC('month', date) ORDER BY date"
            }
            DateInterval::Year => {
                "SELECT term, CAST(DATE_TRUNC('year', date) AS DATE) AS date, \
                 CAST(AVG(yield_return) AS FLOAT4) AS yield_return FROM tyields WHERE date BETWEEN \
                 $1 AND $2 GROUP BY term, DATE_TRUNC('year', date) ORDER BY date"
            }
        };

        let yields: TYields = query_as(query)
            .bind(t_yield_query.start_date())
            .bind(t_yield_query.end_date())
            .fetch_all(self.pool())
            .await?;
        Ok(yields)
    }
}

#[derive(Debug, FromRow)]
struct ZhviMetadataPgRow {
    home_type: HomeType,
    region_type: RegionType,
    region_name: String,
    percentile: Percentile,
}
#[allow(dead_code)]
#[derive(FromRow)]
struct ZhviPricePgRow {
    date: Option<NaiveDate>,
    value: f64,
    // Foreign Key that map to a Zhvi
    home_type: HomeType,
    region_type: RegionType,
    region_name: String,
    percentile: Percentile,
}

impl TryFrom<ZhviPricePgRow> for ZhviPrice {
    type Error = Error;

    fn try_from(row: ZhviPricePgRow) -> Result<Self, Self::Error> {
        let date = row
            .date
            .ok_or(Error::Database("Date Not Found".to_string()))?;
        let value = row.value;
        Ok(Self { date, value })
    }
}

#[async_trait]
impl ZhviPersist for PostgresClient {
    async fn create_zhvi(&self, zhvi: &Zhvi) -> Result<(), Error> {
        let mut tx = self.pool().begin().await?;

        let home_type = zhvi.home_type();
        let region_type = zhvi.region_type();
        let region_name = zhvi.region_name();
        let percentile = zhvi.percentile();
        query!(
            r#"
                INSERT INTO zhvi_metadata
                (region_name, region_type, home_type, percentile)
                VALUES ($1, $2, $3, $4)
            "#,
            region_name,
            region_type as _,
            home_type as _,
            percentile as _
        )
        .execute(&mut *tx)
        .await?;

        for price in zhvi.prices() {
            query!(
                r#"
                    INSERT INTO zhvi_prices
                    (region_name, region_type, home_type, percentile, date, value)
                    VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                region_name,
                region_type as _,
                home_type as _,
                percentile as _,
                price.date as _,
                price.value as _
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }

    async fn read_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<Zhvi, Error> {
        let mut tx = self.pool().begin().await?;
        let metadata = query_as!(
            ZhviMetadataPgRow,
            r#"
                SELECT region_name, region_type AS "region_type: RegionType", home_type AS "home_type: HomeType", percentile AS "percentile: Percentile"
                FROM zhvi_metadata
                WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
            "#,
            id.0,
            id.1 as _,
            id.2 as _,
            id.3 as _,
        )
        .fetch_one(&mut *tx)
        .await?;

        let prices = query_as!(
            ZhviPricePgRow,
            r#"
                SELECT region_name, region_type AS "region_type: RegionType", home_type AS "home_type: HomeType", percentile AS "percentile: Percentile", date, value
                FROM zhvi_prices
                WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
            "#,
            id.0,
            id.1 as _,
            id.2 as _,
            id.3 as _,
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(ZhviPrice::try_from)
        .collect::<Result<ZhviPrices, Error>>()?;

        // Commit the transaction
        tx.commit().await?;

        // Combine metadata and prices to construct Zhvi
        let zhvi = Zhvi {
            home_type: metadata.home_type,
            region_type: metadata.region_type,
            region_name: metadata.region_name,
            percentile: metadata.percentile,
            prices,
        };

        Ok(zhvi)
    }

    async fn update_zhvi(&self, zhvi: &Zhvi) -> Result<(), Error> {
        let mut tx = self.pool().begin().await?;
        let home_type = zhvi.home_type();
        let region_type = zhvi.region_type();
        let region_name = zhvi.region_name();
        let percentile = zhvi.percentile();

        // TODO: Should be updating price(s), not a key
        query!(
            r#"
                UPDATE zhvi_metadata
                SET percentile = $1 WHERE home_type = $2 AND region_type = $3 AND region_name = $4
            "#,
            percentile as _,
            home_type as _,
            region_type as _,
            region_name
        )
        .execute(&mut *tx)
        .await?;

        // Delete existing prices
        query!(
            r#"
                DELETE FROM zhvi_prices
                WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
            "#,
            region_name,
            region_type as _,
            home_type as _,
            percentile as _
        )
        .execute(&mut *tx)
        .await?;

        // Insert updated prices
        for price in zhvi.prices() {
            query!(
                r#"
                    INSERT INTO zhvi_prices
                    (region_name, region_type, home_type, percentile, date, value)
                    VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                region_name,
                region_type as _,
                home_type as _,
                percentile as _,
                &price.date as _,
                &price.value as _
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn delete_zhvi_by_id(&self, id: (&str, &str, &str, &str)) -> Result<(), Error> {
        let mut tx = self.pool().begin().await?;

        query!(
            r#"
                DELETE FROM zhvi_prices
                WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
            "#,
            id.0,
            id.1 as _,
            id.2 as _,
            id.3 as _,
        )
        .execute(&mut *tx)
        .await?;

        query!(
            r#"
                DELETE FROM zhvi_metadata
                WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
            "#,
            id.0,
            id.1 as _,
            id.2 as _,
            id.3 as _,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn read_zhvi_by_query(&self, query: &ZhviQuery) -> Result<Zhvis, Error> {
        let mut tx = self.pool().begin().await?;

        let metadata = query_as!(
            ZhviMetadataPgRow,
            r#"
                SELECT region_name, region_type AS "region_type: RegionType", home_type AS "home_type: HomeType", percentile AS "percentile: Percentile"
                FROM zhvi_metadata
                WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
            "#,
            query.region_name(),
            query.region_type() as _,
            query.home_type() as _,
            query.percentile() as _,
        )
        .fetch_all(&mut *tx)
        .await?;

        let mut zhvis = vec![];
        for metadata in metadata {
            let prices = if query.date_interval() == &DateInterval::Month {
                sqlx::query_as!(
                    ZhviPricePgRow,
                    r#"
                        SELECT region_name, region_type AS "region_type: RegionType", home_type AS "home_type: HomeType", percentile AS "percentile: Percentile", date, value
                        FROM zhvi_prices
                        WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
                        AND date >= $5 AND date <= $6
                    "#,
                    query.region_name(),
                    query.region_type() as _,
                    query.home_type() as _,
                    query.percentile() as _,
                    query.start_date(),
                    query.end_date(),
                )
                .fetch_all(&mut *tx)
                .await?
                .into_iter()
                .map(ZhviPrice::try_from)
                .collect::<Result<ZhviPrices, Error>>()?
            } else if query.date_interval() == &DateInterval::Year {
                query_as!(
                    ZhviPricePgRow,
                    r#"
                        SELECT region_name, region_type AS "region_type: RegionType", home_type AS "home_type: HomeType", percentile AS "percentile: Percentile", date, value
                        FROM zhvi_prices
                        WHERE region_name = $1 AND region_type = $2 AND home_type = $3 AND percentile = $4
                        AND date >= $5 AND date <= $6
                        AND EXTRACT(MONTH FROM date) = 1
                    "#,
                    query.region_name(),
                    query.region_type()as _,
                    query.home_type() as _,
                    query.percentile() as _,
                    query.start_date(),
                    query.end_date(),
                )
                .fetch_all(&mut *tx)
                .await?
                .into_iter()
                .map(ZhviPrice::try_from)
                .collect::<Result<ZhviPrices, Error>>()?
            } else {
                return Err(Error::Parse("Prices not found".to_string()));
            };

            let zhvi = Zhvi {
                home_type: metadata.home_type,
                region_type: metadata.region_type,
                region_name: metadata.region_name,
                percentile: metadata.percentile,
                prices,
            };
            zhvis.push(zhvi);
        }

        tx.commit().await?;

        Ok(zhvis)
    }
}
