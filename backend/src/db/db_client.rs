use sqlx::PgPool;

pub struct DbClient {
    db: PgPool,
}

impl DbClient {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
