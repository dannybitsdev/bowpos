use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("resource already exists")]
    AlreadyExists,
    #[error("resource not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("unauthorized")]
    Unauthorized,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("too many requests")]
    TooManyRequests,
    #[error("internal error")]
    Internal,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidCredentials | AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::AlreadyExists | AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
