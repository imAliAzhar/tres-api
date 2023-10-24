pub type Db = sqlx::PgPool;

use std::env;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Self {
        dotenv().ok();

        Self {
            db: PgPoolOptions::new()
                .connect(&env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        }
    }

    pub fn db(&self) -> &Db {
        &self.db
    }
}
