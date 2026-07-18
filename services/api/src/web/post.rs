use crate::{
    config::Config,
    dtos::{CreatePostRequest, PaginationRequest, PostResponse},
    error::AppError,
    model::Post,
};
use axum::{
    Json,
    extract::{Extension, Path, Query, State},
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_post(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>, AppError> {
    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&user_id)
    .bind(&payload.title)
    .bind(&payload.body)
    .fetch_one(&state.db_pool)
    .await?;

    Ok(Json(PostResponse {
        id: post.id,
        user_id: post.user_id,
        title: post.title,
        body: post.body,
        created_at: post.created_at,
    }))
}

pub async fn get_posts(
    State(state): State<Arc<Config>>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<Json<Vec<PostResponse>>, AppError> {
    let limit = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);

    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db_pool)
    .await?;

    let response = posts
        .into_iter()
        .map(|p| PostResponse {
            id: p.id,
            user_id: p.user_id,
            title: p.title,
            body: p.body,
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(response))
}

pub async fn get_post_by_id(
    State(state): State<Arc<Config>>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>, AppError> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(AppError::BadRequest("Post not found".to_string()))?;

    Ok(Json(PostResponse {
        id: post.id,
        user_id: post.user_id,
        title: post.title,
        body: post.body,
        created_at: post.created_at,
    }))
}
