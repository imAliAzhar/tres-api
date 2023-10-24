use crate::Result;
use axum::{http::StatusCode, Json};

pub type ApiResponse<T> = Result<(StatusCode, Json<T>)>;
