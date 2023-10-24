use std::{env, fmt};

use http;

#[derive(Debug)]
pub enum Error {
    DatabaseError(sqlx::Error),
    HttpError(http::Error),
    EnvError(env::VarError),
    UrlError(url::ParseError),
    RequestError(reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DatabaseError(_) => write!(f, "DatabaseError"),
            Error::HttpError(_) => write!(f, "HttpError"),
            Error::EnvError(e) => e.fmt(f),
            Error::UrlError(e) => e.fmt(f),
            Error::RequestError(e) => e.fmt(f),
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

impl From<env::VarError> for Error {
    fn from(error: env::VarError) -> Self {
        Error::EnvError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::UrlError(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}
