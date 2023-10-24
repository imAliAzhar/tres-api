use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use vercel_runtime::{Body, Error, Response, StatusCode};

use crate::config::config;
use crate::model::{ModelManager, Record, RecordMc};

#[derive(Serialize)]
pub struct ListRecordResponse {
    data: Vec<Record>,
}

pub async fn handler(mm: &ModelManager) -> Result<Response<Body>, Error> {
    let records = get_airtable_records().await?;
    let result = RecordMc::insert_many(mm, &records).await?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(json!({ "data": result }).to_string().into())?)
}

async fn get_airtable_records() -> crate::prelude::Result<Vec<Record>> {
    let client = Client::new();
    get_airtable_records_with_client(&client, None).await
}

#[async_recursion]
async fn get_airtable_records_with_client(
    client: &Client,
    offset: Option<String>,
) -> crate::prelude::Result<Vec<Record>> {
    let url = config().AIRTABLE_BASE_URL.to_string() + "/listRecords";

    let response = client
        .post(&url)
        .header(
            "Authorization",
            format!("Bearer {}", &config().AIRTABLE_API_KEY),
        )
        .json(&json!({ "offset": offset }))
        .send()
        .await?
        .error_for_status()?;

    let response = response.json::<AirtableListResponse>().await?;

    let mut records = response
        .records
        .iter()
        .map(|r| r.fields.date.map(Record::from))
        .filter(|r| r.is_some())
        .flatten()
        .collect::<Vec<Record>>();

    if response.offset.is_some() {
        let more_records = get_airtable_records_with_client(client, response.offset).await?;
        records.extend(more_records);

        return Ok(records);
    }

    Ok(records)
}

#[derive(Deserialize)]
struct AirtableListResponse {
    records: Vec<AirtableRow>,
    offset: Option<String>,
}

#[derive(Deserialize)]
struct AirtableRow {
    fields: Fields,
}

#[derive(Deserialize)]
struct Fields {
    date: Option<DateTime<Utc>>,
}
