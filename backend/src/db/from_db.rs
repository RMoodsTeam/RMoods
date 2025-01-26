use crate::db::db_error::DbError;
use sqlx::PgPool;

/// Trait for converting a database model to the corresponding domain type, i.e. Rust struct.
pub(super) trait FromDb: Sized {
    type DbModel;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, DbError>;
}
