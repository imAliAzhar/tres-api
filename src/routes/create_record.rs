use axum::http::StatusCode;
use axum::{debug_handler, extract::State, Json};
use serde::Serialize;

use crate::base::web::ApiResponse;
use crate::model::{ModelManager, Record, RecordMc};

#[derive(Serialize)]
pub struct CreateRecordResponse {
    data: Record,
}

#[debug_handler]
pub async fn handler(State(mm): State<ModelManager>) -> ApiResponse<CreateRecordResponse> {
    let record = RecordMc::create_record(&mm).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateRecordResponse { data: record }),
    ))
}
