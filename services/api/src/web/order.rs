use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    config::Config,
    dtos::{OrderResponse, PaginatedResponse, PaginationRequest},
    error::AppError,
    grpc_order_client,
};

// GET /orders - list the authenticated user's orders
pub async fn get_orders(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<OrderResponse>>, AppError> {
    let limit = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);

    let (orders, total) =
        grpc_order_client::list_orders(&state.order_client, user_id, limit, offset).await?;

    let items = orders
        .into_iter()
        .map(grpc_order_client::to_order_response)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(PaginatedResponse { items, total }))
}

// GET /orders/{id} - fetch a single order, scoped to the authenticated user
pub async fn get_order_by_id(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = grpc_order_client::get_order(&state.order_client, id).await?;
    let order_response = grpc_order_client::to_order_response(order)?;

    // The Order service itself performs no auth (only reachable on localhost) —
    // the API is the sole authorization boundary, so ownership is checked here.
    if order_response.user_id != user_id {
        return Err(AppError::BadRequest("Order not found".to_string()));
    }

    Ok(Json(order_response))
}
