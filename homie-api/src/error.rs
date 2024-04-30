use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    InvalidQuery(String),
    Fetch(String),
    Serve(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidQuery(err) => (StatusCode::BAD_REQUEST, err),
            AppError::Fetch(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            AppError::Serve(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(_: chrono::ParseError) -> Self {
        AppError::InvalidQuery("Failed to parse from \"%Y-%m-%d\"".to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(_: std::io::Error) -> Self {
        AppError::Serve("Failed to start server".to_string())
    }
}

impl From<homie_core::error::Error> for AppError {
    fn from(_: homie_core::error::Error) -> Self {
        AppError::Fetch("Something unexpected happened when fetching the data".to_string())
    }
}
