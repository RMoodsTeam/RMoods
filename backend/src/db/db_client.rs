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

    pub async fn save(&self, item: &impl DbStored) -> Result<(), sqlx::Error> {
        item.save(&self.db).await
    }

    pub async fn delete(&self, item: &impl DbStored) -> Result<(), sqlx::Error> {
        item.delete(&self.db).await
    }

    pub async fn update(&self, item: &impl DbStored) -> Result<(), sqlx::Error> {
        item.update(&self.db).await
    }

    pub fn raw_db(&self) -> &PgPool {
        &self.db
    }
}
