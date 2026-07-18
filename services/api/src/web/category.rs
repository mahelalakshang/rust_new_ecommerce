use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};

use crate::{
    config::Config,
    dtos::{CategoryResponse, CreateCategoryRequest},
    error::AppError,
    model::Category,
};
use uuid::Uuid;

pub async fn create_category(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<CategoryResponse>, AppError> {
    let category =
        sqlx::query_as::<_, Category>("INSERT INTO categories (name) VALUES ($1) RETURNING *")
            .bind(&payload.name)
            .fetch_one(&state.db_pool)
            .await?;

    Ok(Json(CategoryResponse {
        id: category.id,
        name: category.name.clone(),
        created_at: category.created_at,
    }))
}
