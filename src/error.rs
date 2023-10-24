use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum Error {
    GeneralDatabaseError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        eprintln!("Error: {}", error);
        dbg!(error);
        Error::GeneralDatabaseError
    }
}
