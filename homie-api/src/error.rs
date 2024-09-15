use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::trace_err;

#[derive(Debug, ToSchema)]
pub enum AppError {
    Start(String),
    Fetch(String),
    Request(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // Maps AppError => ( status_code and error_message )
            AppError::Start(err) => (StatusCode::IM_A_TEAPOT, err),
            AppError::Fetch(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            AppError::Request(err) => (StatusCode::BAD_REQUEST, err),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        trace_err!(AppError::Request, "Failed to parse from \"%Y-%m-%d\"", err)
    }
}

impl From<homie_core::error::DomainError> for AppError {
    fn from(err: homie_core::error::DomainError) -> Self {
        trace_err!(
            AppError::Fetch,
            "Something unexpected happened when fetching the data",
            err
        )
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        trace_err!(AppError::Request, "Failed to convert json", err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        trace_err!(AppError::Start, "Failed to start server", err)
    }
}

impl From<tracing_subscriber::filter::ParseError> for AppError {
    fn from(err: tracing_subscriber::filter::ParseError) -> Self {
        trace_err!(AppError::Start, "Failed to parse EnvFilter log", err)
    }
}

#[macro_export]
macro_rules! trace_err {
    ($err_type:expr, $error_msg:expr, $err:expr) => {{
        tracing::error!("{}", $err);
        $err_type($error_msg.to_string())
    }};
}
