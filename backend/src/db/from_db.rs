use axum::async_trait;
use sqlx::{Error, Postgres, Transaction};

/// Trait for converting a database model to the corresponding domain type, i.e. Rust struct.
#[async_trait]
pub(in crate::db) trait FromDb: Sized {
    type DbModel;
    async fn from_db_model(
        model: Self::DbModel,
        tx: &mut Transaction<Postgres>,
    ) -> Result<Self, Error>;
}
