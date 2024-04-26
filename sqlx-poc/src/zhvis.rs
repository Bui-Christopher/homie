use std::error::Error;

use chrono::NaiveDate;
use sqlx::{query, query_as, FromRow, Pool, Postgres};

#[derive(Debug, FromRow)]
pub struct Zhvi {
    pub home_type: String,   // AllHomes/CondoCoOps/SingleFamilyHomes
    pub region_type: String, // Zipcode, City, County
    pub region_name: String,
    pub percentile: String, // Bottom, Middle, Top
    pub prices: Prices,
}

#[derive(Debug, FromRow)]
pub struct ZhviMetadata {
    pub home_type: String,
    pub region_type: String,
    pub region_name: String,
    pub percentile: String,
}

#[derive(Debug, FromRow)]
pub struct ZhviPrice {
    pub date: Option<NaiveDate>,
    pub value: f64,
    // Fields map to a Zhvi
    pub home_type: String,
    pub region_type: String,
    pub region_name: String,
    pub percentile: String,
}

pub type Prices = Vec<ZhviPrice>;

#[derive(Debug)]
pub struct ZhviQuery {
    // Prices
    start_date: NaiveDate,
    end_date: NaiveDate,
    interval_date: String, // Monthly or Yearly
    home_type: String,
    region_type: String,
    region_name: String,
    percentile: String,
}

impl Zhvi {
    pub fn new(date: (i32, u32, u32)) -> Self {
        let date = NaiveDate::from_ymd_opt(date.0, date.1, date.2).unwrap();
        let home_type = "SingleFamilyHomes".to_string();
        let region_type = "City".to_string();
        let region_name = "Irvine".to_string();
        let percentile = "Middle".to_string();
        let price = ZhviPrice {
            date: Some(date),
            value: 100.0,
            home_type: home_type.clone(),
            region_type: region_type.clone(),
            region_name: region_name.clone(),
            percentile: percentile.clone(),
        };
        Self {
            home_type,
            region_type,
            region_name,
            percentile,
            prices: vec![price],
        }
    }

    pub fn set_empty_prices(&mut self) {
        self.prices = vec![]
    }

    pub async fn create(&self, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        query!(
            "INSERT INTO zhvi_metadata (home_type, region_type, region_name, percentile) 
            VALUES ($1, $2, $3, $4)",
            self.home_type,
            self.region_type,
            self.region_name,
            self.percentile
        )
        .execute(&mut *tx)
        .await?;

        for price in &self.prices {
            query!(
                "INSERT INTO zhvi_prices (home_type, region_type, region_name, percentile, date, \
                 value) 
                VALUES ($1, $2, $3, $4, $5, $6)",
                &price.home_type,
                &price.region_type,
                &price.region_name,
                &price.percentile,
                &price.date as _,
                &price.value as _
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn read_by_id(
        pool: &Pool<Postgres>,
        home_type: &str,
        region_type: &str,
        region_name: &str,
        percentile: &str,
    ) -> Result<Zhvi, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Query zhvi metadata
        let metadata = query_as!(
            ZhviMetadata,
            "SELECT home_type, region_type, region_name, percentile 
            FROM zhvi_metadata 
            WHERE home_type = $1 AND region_type = $2 AND region_name = $3 AND percentile = $4",
            home_type,
            region_type,
            region_name,
            percentile
        )
        .fetch_one(&mut *tx)
        .await?;

        let prices = query_as!(
            ZhviPrice,
            "SELECT home_type, region_type, region_name, percentile, date, value
            FROM zhvi_prices 
            WHERE home_type = $1 AND region_type = $2 AND region_name = $3 AND percentile = $4",
            home_type,
            region_type,
            region_name,
            percentile
        )
        .fetch_all(&mut *tx)
        .await?;

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

    pub async fn update(&self, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Update metadata
        query!(
            "UPDATE zhvi_metadata SET percentile = $1 WHERE home_type = $2 AND region_type = $3 \
             AND region_name = $4",
            &self.percentile,
            &self.home_type,
            &self.region_type,
            &self.region_name
        )
        .execute(&mut *tx)
        .await?;

        // Delete existing prices
        query!(
            "DELETE FROM zhvi_prices WHERE home_type = $1 AND region_type = $2 AND region_name = \
             $3 AND percentile = $4",
            &self.home_type,
            &self.region_type,
            &self.region_name,
            &self.percentile
        )
        .execute(&mut *tx)
        .await?;

        // Insert updated prices
        for price in &self.prices {
            query!(
                "INSERT INTO zhvi_prices (home_type, region_type, region_name, percentile, date, \
                 value) 
                 VALUES ($1, $2, $3, $4, $5, $6)",
                &price.home_type,
                &price.region_type,
                &price.region_name,
                &price.percentile,
                &price.date as _,
                &price.value as _
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    pub async fn delete_by_id(
        pool: &Pool<Postgres>,
        home_type: &str,
        region_type: &str,
        region_name: &str,
        percentile: &str,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        query!(
            "DELETE FROM zhvi_metadata WHERE home_type = $1 AND region_type = $2 AND region_name \
             = $3 AND percentile = $4",
            home_type,
            region_type,
            region_name,
            percentile
        )
        .execute(&mut *tx)
        .await?;

        query!(
            "DELETE FROM zhvi_prices WHERE home_type = $1 AND region_type = $2 AND region_name = \
             $3 AND percentile = $4",
            home_type,
            region_type,
            region_name,
            percentile
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn read_by_query(
        pool: &Pool<Postgres>,
        query: ZhviQuery,
    ) -> Result<Vec<Zhvi>, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let metadata = query_as!(
            ZhviMetadata,
            "SELECT home_type, region_type, region_name, percentile 
            FROM zhvi_metadata 
            WHERE home_type = $1 AND region_type = $2 AND region_name = $3 AND percentile = $4",
            query.home_type,
            query.region_type,
            query.region_name,
            query.percentile
        )
        .fetch_all(&mut *tx)
        .await?;

        let mut zhvis = vec![];
        for metadata in metadata {
            let prices = if query.interval_date == "Monthly" {
                sqlx::query_as!(
                    ZhviPrice,
                    "SELECT home_type, region_type, region_name, percentile, date, value
                     FROM zhvi_prices 
                     WHERE home_type = $1 AND region_type = $2 AND region_name = $3 AND percentile \
                     = $4
                     AND date >= $5 AND date <= $6",
                    query.home_type,
                    query.region_type,
                    query.region_name,
                    query.percentile,
                    query.start_date,
                    query.end_date
                )
                .fetch_all(&mut *tx)
                .await?
            } else if query.interval_date == "Yearly" {
                query_as!(
                    ZhviPrice,
                    "SELECT home_type, region_type, region_name, percentile, date, value
                    FROM zhvi_prices WHERE home_type = $1 AND region_type = $2
                    AND region_name = $3 AND percentile = $4 AND EXTRACT(MONTH FROM date) = 1
                    AND date >= $5 AND date <= $6",
                    query.home_type,
                    query.region_type,
                    query.region_name,
                    query.percentile,
                    query.start_date,
                    query.end_date
                )
                .fetch_all(&mut *tx)
                .await?
            } else {
                return Err(sqlx::Error::RowNotFound);
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
impl ZhviQuery {
    pub fn new(
        start_date: NaiveDate,
        end_date: NaiveDate,
        interval_date: String,
        home_type: String,
        region_type: String,
        region_name: String,
        percentile: String,
    ) -> Self {
        ZhviQuery {
            start_date,
            end_date,
            interval_date,
            home_type,
            region_type,
            region_name,
            percentile,
        }
    }
}

pub async fn test_zhvis(pool: &Pool<Postgres>) -> Result<(), Box<dyn Error>> {
    // Create
    let zhvi = Zhvi::new((2001, 2, 2));
    zhvi.create(pool).await?;

    // Read
    let home_type = "SingleFamilyHomes";
    let region_type = "City";
    let region_name = "Irvine";
    let percentile = "Middle";
    let mut zhvi = Zhvi::read_by_id(pool, home_type, region_type, region_name, percentile).await?;
    println!("{zhvi:#?}");

    // Update
    zhvi.set_empty_prices();
    zhvi.update(pool).await?;
    let zhvi = Zhvi::read_by_id(pool, home_type, region_type, region_name, percentile).await?;
    println!("{zhvi:#?}");

    // Read By Query
    let zhvi_query = ZhviQuery::new(
        NaiveDate::from_ymd_opt(2000, 2, 2).unwrap(),
        NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
        "Yearly".to_string(),
        "SingleFamilyHomes".to_string(),
        "City".to_string(),
        "Irvine".to_string(),
        "Middle".to_string(),
    );
    let zhvi = Zhvi::read_by_query(pool, zhvi_query).await?;
    println!("{zhvi:#?}");

    // Delete
    Zhvi::delete_by_id(pool, home_type, region_type, region_name, percentile).await?;

    Ok(())
}
