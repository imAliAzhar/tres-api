use axum::extract::Query;
use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::base::Pagination;
use crate::model::{ModelManager, Record, RecordMc};
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct ListRecordsQuery {
    pg_num: Option<i32>,
    pg_size: Option<i32>,
}

#[derive(Serialize)]
pub struct ListRecordResponse {
    data: Vec<Record>,
}

#[debug_handler]
pub async fn handler(
    State(mm): State<ModelManager>,
    Query(params): Query<ListRecordsQuery>,
) -> Result<Json<ListRecordResponse>> {
    let ListRecordsQuery { pg_num, pg_size } = params;

    let pagination = Pagination::new(pg_num, pg_size);

    let result = RecordMc::list_records(&mm, pagination).await?;

    Ok(Json(ListRecordResponse { data: result }))
}
