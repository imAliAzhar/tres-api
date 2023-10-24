pub type Db = sqlx::PgPool;

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &Db {
        &self.db
    }
}
