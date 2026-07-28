use axum::{
    http::{header, HeaderValue, Method},
    middleware::from_fn_with_state,
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod dtos;
mod error;
mod grpc_client;
mod model;
mod utils;
mod web;

use config::Config;
use web::{
    auth, cart as cart_handler, category as category_handler, mw, post as post_handler,
    product as product_handler,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration //Arc only clone pointers without clone strings many times
    let config = Config::from_env().await?;
    let cors_origin = config.cors_origin.clone();
    let state = Arc::new(config);

    // Run migrations on startup to ensure consistency
    sqlx::migrate!("./migrations")
        .run(&state.db_pool)
        .await
        .expect("Failed to run migrations");

    // Auth Routes
    let auth_routes = Router::new()
        .route("/signup", post(auth::signup_handler))
        .route("/login", post(auth::login_handler));

    // Post Routes (Protected)
    let post_routes = Router::new()
        .route(
            "/",
            post(post_handler::create_post).get(post_handler::get_posts),
        )
        .route("/{id}", get(post_handler::get_post_by_id))
        .route_layer(from_fn_with_state(state.clone(), mw::auth_guard));

    // Product Routes: read requires auth; create requires auth + admin
    let product_routes = Router::new()
        .merge(
            Router::new()
                .route("/", post(product_handler::create_product))
                .route_layer(from_fn_with_state(state.clone(), mw::require_admin))
                .route_layer(from_fn_with_state(state.clone(), mw::auth_guard)),
        )
        .merge(
            Router::new()
                .route("/", get(product_handler::get_products))
                .route("/{id}", get(product_handler::get_product_by_id))
                .route_layer(from_fn_with_state(state.clone(), mw::auth_guard)),
        );

    //Category Routes
    let category_routes = Router::new()
        .route("/", post(category_handler::create_category))
        .route_layer(from_fn_with_state(state.clone(), mw::auth_guard));

    // Cart Routes (Protected)
    let cart_routes = Router::new()
        .route(
            "/",
            get(cart_handler::get_cart).post(cart_handler::add_to_cart),
        )
        .route(
            "/{product_id}",
            patch(cart_handler::update_cart_item).delete(cart_handler::remove_from_cart),
        )
        .route_layer(from_fn_with_state(state.clone(), mw::auth_guard));

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(
            cors_origin
                .parse::<HeaderValue>()
                .expect("CORS_ORIGIN must be a valid origin value"),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    // Combine Routes
    let app = Router::new()
        .nest("/auth", auth_routes)
        .nest("/posts", post_routes)
        .nest("/products", product_routes)
        .nest("/categories", category_routes)
        .nest("/cart", cart_routes)
        .with_state(state)
        .layer(cors);

    // Start Server
    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
