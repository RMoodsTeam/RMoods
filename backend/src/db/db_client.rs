use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct DbClient {
    db: PgPool,
}

impl DbClient {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub(super) fn raw_db(&self) -> &PgPool {
        &self.db
    }
}
