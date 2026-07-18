use crate::{
    config::Config,
    dtos::{AuthResponse, LoginRequest, SignupRequest},
    error::AppError,
    model::User,
    utils::{
        hash::{hash_password, verify_password},
        jwt::encode_jwt,
    },
};
use axum::{Json, extract::State};
use std::sync::Arc;

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

    let token = encode_jwt(user.id, &state.jwt_secret)?;

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

    let token = encode_jwt(user.id, &state.jwt_secret)?;

    Ok(Json(AuthResponse { token }))
}
