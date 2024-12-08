use sqlx::PgPool;

/// The database client.
///
/// It doesn't do much, but may come in handy down the road.
/// For now, it only holds the underlying database pool and provides a way to access it for the internals of this module.
#[derive(Clone, Debug)]
pub struct DbClient {
    db: PgPool,
}

impl DbClient {
    /// Create a new instance
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Get a reference to the underlying database pool
    pub(super) fn raw_db(&self) -> &PgPool {
        &self.db
    }
}
