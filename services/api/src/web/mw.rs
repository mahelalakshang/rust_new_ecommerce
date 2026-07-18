use crate::config::Config;
use crate::error::AppError;
use crate::utils::jwt::decode_jwt;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn auth_guard(
    State(state): State<Arc<Config>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .ok_or(AppError::Unauthorized)?;

    let claims = decode_jwt(&token, &state.jwt_secret)?;

    // Insert user_id into request extensions for handlers to use
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}

/// Runs after [`auth_guard`]. Rejects non-admins with 403.
pub async fn require_admin(
    State(state): State<Arc<Config>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .copied()
        .ok_or(AppError::Unauthorized)?;

    let role: Option<String> = sqlx::query_scalar("SELECT role FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await?;

    match role.as_deref() {
        Some("admin") => Ok(next.run(req).await),
        Some(_) => Err(AppError::Forbidden),
        None => Err(AppError::BadRequest("User not found".to_string())),
    }
}
