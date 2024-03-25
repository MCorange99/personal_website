use anyhow::bail;
use sqlx::{postgres::PgPoolOptions, Postgres};

use crate::config::definition::Config;


pub mod models;


#[derive(Debug, Clone)]
pub struct Database {
    connection: sqlx::Pool<Postgres>
}


impl Database {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        log::info!("Database connecting to {}", config.database.url);
        let conn = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database.url).await;


        match conn {
            Ok(c) => {
                log::info!("Connection successfull");
                Ok(Self {
                    connection: c
                })
            },
            Err(e) => {
                log::error!("Failed to connect to database: {e}");
                bail!(e)
            },
        }
    }

    pub fn connection(&self) -> &sqlx::Pool<Postgres> {
        &self.connection
    }
}