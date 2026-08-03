use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Unauthorized,
    Forbidden,
    BadRequest(String),
    InternalServerError,
}

impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        AppError::Database(inner)
    }
}

impl From<tonic::Status> for AppError {
    fn from(status: tonic::Status) -> Self {
        use tonic::Code;
        match status.code() {
            Code::InvalidArgument | Code::FailedPrecondition | Code::NotFound => {
                AppError::BadRequest(status.message().to_string())
            }
            _ => {
                tracing::error!("Order service error: {:?}", status);
                AppError::InternalServerError
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database Error".to_string(),
                )
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
