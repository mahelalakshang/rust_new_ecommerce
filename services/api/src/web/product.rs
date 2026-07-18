use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};

use crate::{
    config::Config,
    dtos::{CreateProductRequest, PaginationRequest, ProductResponse},
    error::AppError,
    model::{Product, User},
};
use uuid::Uuid;

pub async fn create_product(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>, AppError> {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (user_id, category_id, name, description, price, stock_quantity) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&user_id)
    .bind(&payload.category_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.price)
    .bind(&payload.stock_quantity)
    .fetch_one(&state.db_pool)
    .await?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&user_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(AppError::BadRequest("User not found".to_string()))?;

    println!("{:#?}", user.username);

    // Send notification to the notification service
    let notification_result = crate::grpc_client::send_product_notification(
        &user_id.to_string(),
        &product.name,
        &user.username,
    )
    .await;

    // Log if notification fails, but don't fail the product creation
    if let Err(e) = notification_result {
        tracing::warn!("Failed to send notification: {}", e);
    }

    Ok(Json(ProductResponse {
        id: product.id,
        category_id: product.category_id,
        name: product.name.clone(),
        description: product.description,
        price: product.price,
        stock_quantity: product.stock_quantity,
    }))
}

pub async fn get_products(
    State(state): State<Arc<Config>>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<Vec<ProductResponse>>, AppError> {
    let limit = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);

    let products = sqlx::query_as::<_, Product>(
        "SELECT * FROM products ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await?;

    let response = products
        .into_iter()
        .map(|p| ProductResponse {
            id: p.id,
            category_id: p.category_id,
            name: p.name,
            description: p.description,
            price: p.price,
            stock_quantity: p.stock_quantity,
        })
        .collect();

    Ok(Json(response))
}

pub async fn get_product_by_id(
    State(state): State<Arc<Config>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductResponse>, AppError> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(AppError::BadRequest("Product not found".to_string()))?;

    Ok(Json(ProductResponse {
        id: product.id,
        category_id: product.category_id,
        name: product.name,
        description: product.description,
        price: product.price,
        stock_quantity: product.stock_quantity,
    }))
}
