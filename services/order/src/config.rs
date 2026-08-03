use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tracing::info;

#[derive(Clone)]
pub struct Config {
    pub db_pool: PgPool,
}

impl Config {
    pub async fn from_env() -> Result<Self, sqlx::Error> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        info!("Database connected");

        Ok(Self { db_pool: pool })
    }
}
