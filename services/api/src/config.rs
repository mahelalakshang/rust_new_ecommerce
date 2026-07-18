use std::env;
use std::sync::Arc;
use sqlx::postgres::{PgPoolOptions, PgPool};
use dotenvy::dotenv;
use tracing::info;

#[derive(Clone)]
pub struct Config {
    pub db_pool: PgPool,
    pub jwt_secret: String,
}

impl Config {
    pub async fn from_env() -> Result<Self, sqlx::Error> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        info!("Database connected");

        Ok(Self {
            db_pool: pool,
            jwt_secret,
        })
    }
}
