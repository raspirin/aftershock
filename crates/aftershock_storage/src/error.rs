use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database Pool Error: {0}")]
    DatabasePoolError(#[from] r2d2::Error),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let msg = format!("{self}");

        (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
    }
}