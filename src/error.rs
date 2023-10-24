use std::fmt;

use http;

#[derive(Debug)]
pub enum Error {
    DatabaseError(sqlx::Error),
    HttpError(http::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DatabaseError(_) => write!(f, "DatabaseError"),
            Error::HttpError(_) => write!(f, "HttpError"),
        }
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Error::DatabaseError(error)
    }
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        Error::HttpError(error)
    }
}
