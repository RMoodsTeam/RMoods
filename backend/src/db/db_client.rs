use crate::db::db_stored::DbStored;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct DbClient {
    db: PgPool,
}

impl DbClient {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn save(&self, item: impl DbStored) {
        item.save(&self.db).await.unwrap();
    }

    pub async fn delete(&self, item: impl DbStored) {
        item.delete(&self.db).await.unwrap();
    }

    pub async fn update(&self, item: impl DbStored) {
        item.update(&self.db).await.unwrap();
    }

    // TODO: REMOVE THIS
    pub fn raw_db(&self) -> &PgPool {
        &self.db
    }
}
