use sqlx::{postgres::PgPoolOptions, Postgres};

use crate::config::definition::Config;


pub mod models;


#[derive(Debug, Clone)]
pub struct Database {
    connection: sqlx::Pool<Postgres>
}


impl Database {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let conn = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database.url).await?;
        Ok(Self {
            connection: conn
        })
    }

    pub fn connection(&self) -> &sqlx::Pool<Postgres> {
        &self.connection
    }
}