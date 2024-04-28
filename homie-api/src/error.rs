use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub enum AppError {
    QueryParamParse(String),
    Database(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::QueryParamParse(err) => (StatusCode::BAD_REQUEST, err),
            AppError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(_value: chrono::ParseError) -> Self {
        AppError::QueryParamParse("Failed to parse from time input".to_string())
    }
}

impl From<homie_core::error::Error> for AppError {
    fn from(_value: homie_core::error::Error) -> Self {
        AppError::Database("Something unexpected happened when reading data".to_string())
    }
}
