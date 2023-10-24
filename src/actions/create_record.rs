use serde::Serialize;
use serde_json::json;
use vercel_runtime::{Body, Error, Response, StatusCode};

use crate::model::{ModelManager, Record, RecordMc};

#[derive(Serialize)]
pub struct CreateRecordResponse {
    data: Record,
}

pub async fn handler(mm: &ModelManager) -> Result<Response<Body>, Error> {
    let record = RecordMc::create_record(mm).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({ "data": record }).to_string().into())?)
}
