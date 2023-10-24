#![allow(dead_code, unused)]
use anyhow::Result;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

const DATABASE_URL: &str = "postgres://postgres:1234@localhost/postgres";
const SQL_DIR: &str = "tests/sql";

type Db = Pool<Postgres>;

#[tokio::test]
async fn dev() -> Result<()> {
    // init_dev_db().await.unwrap();

    // let hc = httpc_test::new_client("http://localhost:3000/api")?;

    // hc.do_post("/records", json!({})).await?.print().await?;

    // hc.do_get("/records").await?.print().await?;

    // hc.do_post("/seed", json!({})).await?.print().await?;

    Ok(())
}

async fn setup_db(url: &str) -> Db {
    PgPoolOptions::new().connect(url).await.unwrap()
}

async fn init_dev_db() -> Result<()> {
    println!("init_dev_db");
    let current_dir = std::env::current_dir().unwrap();

    println!("{current_dir:?}");
    let sql_dir = current_dir.join(SQL_DIR);

    let mut paths = fs::read_dir(sql_dir)?
        .map(|e| e.unwrap().path())
        .filter(|e| e.to_str().unwrap().ends_with(".sql"))
        .collect::<Vec<PathBuf>>();

    paths.sort();

    println!("{len}", len = paths.len());
    let db = setup_db(DATABASE_URL).await;

    for path in paths {
        execute_sql_file(&db, path).await.unwrap();
    }

    Ok(())
}

async fn execute_sql_file(db: &Db, path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let sql_files: Vec<&str> = content.split(';').collect();

    for sql in sql_files {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}
