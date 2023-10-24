pub type Db = sqlx::PgPool;

use crate::config::config;

use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Self {
        Self {
            db: PgPoolOptions::new()
                .connect(&config().DATABASE_URL)
                .await
                .expect("ERROR: Cannot connect to database"),
        }
    }

    pub fn db(&self) -> &Db {
        &self.db
    }
}
