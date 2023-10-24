use crate::{base::Pagination, prelude::*};

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

use super::ModelManager;

#[derive(FromRow, Serialize)]
pub struct Record {
    date: DateTime<Utc>,
    duration_str: Option<String>,
    duration_ms: Option<i32>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            date: Utc::now(),
            duration_ms: None,
            duration_str: None,
        }
    }
}

impl Default for Record {
    fn default() -> Self {
        Record::new()
    }
}

pub struct RecordMc {}

impl RecordMc {
    pub async fn create_record(mm: &ModelManager) -> Result<Record> {
        let db = mm.db();

        let record = Record::new();

        let result = sqlx::query_as!(
            Record,
            r#"
            INSERT INTO records (date, duration_str, duration_ms)
            VALUES ($1, $2, $3)
            RETURNING date, duration_str, duration_ms;
        "#,
            record.date,
            record.duration_str,
            record.duration_ms
        )
        .fetch_one(db)
        .await?;

        Ok(result)
    }

    pub async fn list_records(mm: &ModelManager, pagination: Pagination) -> Result<Vec<Record>> {
        let db = mm.db();
        let Pagination { pg_num, pg_size } = pagination;

        let offset = pg_num * pg_size;

        let result = sqlx::query_as!(
            Record,
            r#"
            SELECT date, duration_str, duration_ms
            FROM records
            ORDER BY date DESC
            LIMIT $1
            OFFSET $2;
        "#,
            pg_size as i64,
            offset as i64
        )
        .fetch_all(db)
        .await?;

        Ok(result)
    }
}
