use std::env;
use std::sync::Arc;
use sqlx::postgres::{PgPoolOptions, PgPool};
use dotenvy::dotenv;
use tonic::transport::Channel;
use tracing::info;

use crate::grpc_order_client::order::order_service_client::OrderServiceClient;

#[derive(Clone)]
pub struct Config {
    pub db_pool: PgPool,
    pub jwt_secret: String,
    pub cors_origin: String,
    pub order_client: OrderServiceClient<Channel>,
}

impl Config {
    pub async fn from_env() -> Result<Self, sqlx::Error> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let cors_origin =
            env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        info!("Database connected");

        Ok(Self {
            db_pool: pool,
            jwt_secret,
            cors_origin,
            order_client: crate::grpc_order_client::connect(),
        })
    }
}
