use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database Pool Error: {0}")]
    DatabasePoolError(#[from] r2d2::Error),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("Database Migration Error: {0}")]
    MigrationError(#[from] Box<dyn core::error::Error + Send + Sync>),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Wrong content kind literal")]
    ContentKindError,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let msg = format!("{self}");

        match &self {
            Self::NotFound(_) => (StatusCode::NOT_FOUND, msg).into_response(),
            Self::DatabaseError(diesel::result::Error::NotFound) => {
                (StatusCode::NOT_FOUND, "Resource not found".to_string()).into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
