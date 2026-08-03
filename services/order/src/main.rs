use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use order::config::Config;
use order::proto::order::order_service_server::OrderServiceServer;
use order::service::OrderServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env().await?;

    // Shares one Postgres instance (and one `_sqlx_migrations` table) with the API
    // service, which owns an earlier, longer migration history. `ignore_missing`
    // lets this migrator skip over versions it doesn't recognize (the API's)
    // instead of erroring, while still applying its own new migrations.
    let mut migrator = sqlx::migrate!("./migrations");
    migrator.set_ignore_missing(true);
    migrator
        .run(&config.db_pool)
        .await
        .expect("Failed to run migrations");

    let addr = "0.0.0.0:50052".parse()?;
    let order_service = OrderServiceImpl {
        db_pool: config.db_pool,
    };

    tracing::info!("Order service starting on {}", addr);

    Server::builder()
        .add_service(OrderServiceServer::new(order_service))
        .serve(addr)
        .await?;

    Ok(())
}
