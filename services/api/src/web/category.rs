use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};

use crate::{
    config::Config,
    dtos::{CategoryResponse, CreateCategoryRequest, PaginatedResponse, PaginationRequest},
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
            .await
            .map_err(|e| {
                if let Some(db_error) = e.as_database_error() {
                    if db_error.is_unique_violation() {
                        return AppError::BadRequest("Category name already exists".to_string());
                    }
                }
                AppError::Database(e)
            })?;

    Ok(Json(CategoryResponse {
        id: category.id,
        name: category.name.clone(),
        created_at: category.created_at,
    }))
}

pub async fn get_categories(
    State(state): State<Arc<Config>>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<PaginatedResponse<CategoryResponse>>, AppError> {
    let limit = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories")
        .fetch_one(&state.db_pool)
        .await?;

    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await?;

    let items = categories
        .into_iter()
        .map(|c| CategoryResponse {
            id: c.id,
            name: c.name,
            created_at: c.created_at,
        })
        .collect();

    Ok(Json(PaginatedResponse { items, total }))
}

pub async fn get_category_by_id(
    State(state): State<Arc<Config>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryResponse>, AppError> {
    let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(AppError::BadRequest("Category not found".to_string()))?;

    Ok(Json(CategoryResponse {
        id: category.id,
        name: category.name,
        created_at: category.created_at,
    }))
}
