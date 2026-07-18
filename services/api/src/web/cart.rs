use axum::{
    extract::{Path, State},
    Extension, Json,
};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    config::Config,
    dtos::{AddToCartRequest, CartItemResponse, CartResponse, UpdateCartItemRequest},
    error::AppError,
    model::Product,
};

#[derive(sqlx::FromRow)]
struct CartItemDetail {
    id: Uuid,
    product_id: Uuid,
    product_name: String,
    price: Decimal,
    quantity: i32,
}

// GET /cart - Retrieve user's cart
pub async fn get_cart(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<CartResponse>, AppError> {
    let items = sqlx::query_as::<_, CartItemDetail>(
        r#"
        SELECT
            c.id,
            c.product_id,
            p.name as product_name,
            p.price,
            c.quantity
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db_pool)
    .await?;

    let mut total = Decimal::new(0, 0);
    let mut item_responses = Vec::new();

    for item in items {
        let subtotal = item.price * Decimal::from(item.quantity);
        total += subtotal;

        item_responses.push(CartItemResponse {
            id: item.id,
            product_id: item.product_id,
            product_name: item.product_name,
            price: item.price,
            quantity: item.quantity,
            subtotal,
        });
    }

    Ok(Json(CartResponse {
        items: item_responses,
        total,
    }))
}

// POST /cart - Add or update quantity of product in cart
pub async fn add_to_cart(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<AddToCartRequest>,
) -> Result<Json<CartResponse>, AppError> {
    if payload.quantity <= 0 {
        return Err(AppError::BadRequest(
            "Quantity must be greater than 0".to_string(),
        ));
    }

    // 1. Verify product exists and check stock
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(payload.product_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or_else(|| AppError::BadRequest("Product not found".to_string()))?;

    if product.stock_quantity < payload.quantity {
        return Err(AppError::BadRequest("Insufficient stock".to_string()));
    }

    // 2. Insert or update on conflict (Upsert)
    sqlx::query(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, product_id)
        DO UPDATE SET
            quantity = cart_items.quantity + EXCLUDED.quantity,
            updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(payload.product_id)
    .bind(payload.quantity)
    .execute(&state.db_pool)
    .await?;

    // 3. Return the updated cart
    get_cart(State(state), Extension(user_id)).await
}

// PATCH /cart/{product_id} - Directly set item quantity
pub async fn update_cart_item(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Path(product_id): Path<Uuid>,
    Json(payload): Json<UpdateCartItemRequest>,
) -> Result<Json<CartResponse>, AppError> {
    if payload.quantity <= 0 {
        return Err(AppError::BadRequest(
            "Quantity must be greater than 0".to_string(),
        ));
    }

    // Check stock
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(product_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or_else(|| AppError::BadRequest("Product not found".to_string()))?;

    if product.stock_quantity < payload.quantity {
        return Err(AppError::BadRequest("Insufficient stock".to_string()));
    }

    // Update quantity
    sqlx::query(
        "UPDATE cart_items SET quantity = $1, updated_at = NOW() WHERE user_id = $2 AND product_id = $3",
    )
    .bind(payload.quantity)
    .bind(user_id)
    .bind(product_id)
    .execute(&state.db_pool)
    .await?;

    get_cart(State(state), Extension(user_id)).await
}

// DELETE /cart/{product_id} - Remove item from cart
pub async fn remove_from_cart(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<CartResponse>, AppError> {
    sqlx::query("DELETE FROM cart_items WHERE user_id = $1 AND product_id = $2")
        .bind(user_id)
        .bind(product_id)
        .execute(&state.db_pool)
        .await?;

    get_cart(State(state), Extension(user_id)).await
}
