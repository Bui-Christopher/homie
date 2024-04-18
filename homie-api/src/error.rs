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
