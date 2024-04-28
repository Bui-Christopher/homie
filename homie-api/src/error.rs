use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub enum AppError {
    QueryParamParse(String),
    Database(),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::QueryParamParse(err) => (StatusCode::BAD_REQUEST, err),
            AppError::Database() => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

// #[derive(Debug)]
// pub enum ApiError {
//     Auth { status_code: u16, message: String },
//     Db { status_code: u16, message: String },
//     Request { status_code: u16, message: String },
// }

// TokenError(#[from] TokenError),
// UserError(#[from] UserError),
// DbError(#[from] DbError),

// #[derive(Debug, Error)]
// pub enum RequestError {
//     #[error(transparent)]
//     ValidationError(#[from] validator::ValidationErrors),
//     #[error(transparent)]
//     JsonRejection(#[from] JsonRejection),
// }
