use std::env;
use std::error::Error;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::adapter::repository::{Config, Persist};
use crate::domain::hpi::{Hpi, HpiData, HpiPersist, HpiQuery, Hpis};
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
}

impl Persist for PostgresClient {}

impl HpiPersist for PostgresClient {
    fn create_hpi(&self, hpi: &Hpi) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi create for: {:?} from PostgresClient.", hpi);
        Ok(true)
    }

    fn read_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_hpi(&self, hpi: &Hpi) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi update for: {:?} from PostgresClient.", hpi);
        Ok(true)
    }

    fn delete_hpi_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling hpi delete with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn read_hpi_by_query(&self, query: &HpiQuery) -> Result<Hpis, Box<dyn Error>> {
        println!("Calling hpi read by: {:?} from PostgresClient.", query);
        Ok(HpiData::generate_dummy_data())
    }
}

#[async_trait]
impl TYieldPersist for PostgresClient {
    async fn create_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>> {
        let req = sqlx::query!(
            r#"
        INSERT INTO tyields ( t_yield )
        VALUES ( $1 )
        "#,
            Json(t_yield) as _
        )
        .fetch_one(self.pool)
        .await?;
        Ok(true)
    }

    fn read_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield read with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn update_t_yield(&self, t_yield: &TYield) -> Result<bool, Box<dyn Error>> {
        println!(
            "Calling t_yield update for: {:?} from PostgresClient.",
            t_yield
        );
        Ok(true)
    }

    fn delete_t_yield_by_id(&self, id: &str) -> Result<bool, Box<dyn Error>> {
        println!("Calling t_yield delete with id: {id} from PostgresClient.");
        Ok(true)
    }

    fn read_t_yield_by_query(&self, query: &TYieldQuery) -> Result<TYields, Box<dyn Error>> {
        println!("Calling t_yield read by: {:?} from PostgresClient.", query);
        Ok(TYield::generate_dummy_data())
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
