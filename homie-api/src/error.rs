use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::trace_err;

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
    fn from(err: chrono::ParseError) -> Self {
        trace_err!(
            AppError::InvalidQuery,
            "Failed to parse from \"%Y-%m-%d\"",
            err
        )
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        trace_err!(AppError::Serve, "Failed to start server", err)
    }
}

impl From<homie_core::error::Error> for AppError {
    fn from(err: homie_core::error::Error) -> Self {
        trace_err!(
            AppError::Fetch,
            "Something unexpected happened when fetching the data",
            err
        )
    }
}

#[macro_export]
macro_rules! trace_err {
    ($err_type:expr, $error_msg:expr, $err:expr) => {{
        tracing::error!("{}", $err);
        $err_type($error_msg.to_string())
    }};
}
