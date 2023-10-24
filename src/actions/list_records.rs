use serde::Serialize;
use serde_json::json;
use vercel_runtime::{Body, Error, Response, StatusCode};

use crate::base::Pagination;
use crate::model::{ModelManager, Record, RecordMc};

#[derive(Serialize)]
pub struct ListRecordResponse {
    data: Vec<Record>,
}

pub async fn handler(mm: &ModelManager) -> Result<Response<Body>, Error> {
    let pagination = Pagination::default();

    let result = RecordMc::list_records(mm, pagination).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({ "data": result }).to_string().into())?)
}
