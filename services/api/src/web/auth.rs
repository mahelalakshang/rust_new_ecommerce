use crate::{
    config::Config,
    dtos::{AuthResponse, LoginRequest, MeResponse, SignupRequest},
    error::AppError,
    model::User,
    utils::{
        hash::{hash_password, verify_password},
        jwt::encode_jwt,
    },
};
use axum::{Extension, Json, extract::State};
use std::sync::Arc;
use uuid::Uuid;

pub async fn signup_handler(
    State(state): State<Arc<Config>>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let hashed_password = hash_password(&payload.password)?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, password_hash, created_at, role"
    )
    .bind(&payload.username)
    .bind(&hashed_password)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        // Handle unique constraint violation
        if let Some(db_error) = e.as_database_error() {
             if db_error.is_unique_violation() {
                 return AppError::BadRequest("Username already exists".to_string());
             }
        }
        AppError::Database(e)
    })?;

    let token = encode_jwt(user.id, &user.role, &state.jwt_secret)?;

    Ok(Json(AuthResponse { token }))
}

pub async fn login_handler(
    State(state): State<Arc<Config>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = encode_jwt(user.id, &user.role, &state.jwt_secret)?;

    Ok(Json(AuthResponse { token }))
}

pub async fn me_handler(
    State(state): State<Arc<Config>>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<MeResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(AppError::Unauthorized)?;

    Ok(Json(MeResponse {
        id: user.id,
        username: user.username,
        role: user.role,
        created_at: user.created_at,
    }))
}
