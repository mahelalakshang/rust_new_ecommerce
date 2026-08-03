use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use uuid::Uuid;

use crate::{config::Config, dtos::OrderResponse, error::AppError, grpc_order_client, model::User};

#[derive(sqlx::FromRow)]
struct CartLineItem {
    product_id: Uuid,
    quantity: i32,
}

// POST /checkout - Convert the current user's cart into an order via the Order gRPC service
pub async fn checkout(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let cart_items = sqlx::query_as::<_, CartLineItem>(
        "SELECT product_id, quantity FROM cart_items WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.db_pool)
    .await?;

    if cart_items.is_empty() {
        return Err(AppError::BadRequest("Cart is empty".to_string()));
    }

    let items: Vec<(Uuid, i32)> = cart_items
        .into_iter()
        .map(|c| (c.product_id, c.quantity))
        .collect();

    // Price/stock are re-derived server-side inside the Order service's locked
    // transaction — only product_id/quantity ever cross this boundary.
    let order = grpc_order_client::create_order(&state.order_client, user_id, items).await?;
    let order_response = grpc_order_client::to_order_response(order)?;

    // Checkout succeeded: clear the cart now that it's been converted to an order.
    sqlx::query("DELETE FROM cart_items WHERE user_id = $1")
        .bind(user_id)
        .execute(&state.db_pool)
        .await?;

    // Fire-and-forget order confirmation email, same pattern as product creation
    // in web/product.rs: log on failure, never fail the checkout response for it.
    if let Ok(Some(user)) = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await
    {
        let notification_result = crate::grpc_client::send_order_confirmation(
            &user_id.to_string(),
            &order_response.id.to_string(),
            &user.username,
            &order_response.total_amount.to_string(),
        )
        .await;

        if let Err(e) = notification_result {
            tracing::warn!("Failed to send order confirmation: {}", e);
        }
    }

    Ok(Json(order_response))
}
