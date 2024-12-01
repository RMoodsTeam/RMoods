use axum::async_trait;

/// Implemented for structs that can be saved to our database
#[async_trait]
pub trait DbStored {
    type DbStruct;
    async fn save(&self, db: &sqlx::PgPool) -> Result<Self::DbStruct, sqlx::Error>;
    async fn delete(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error>;
    async fn update(&self, db: &sqlx::PgPool) -> Result<Self::DbStruct, sqlx::Error>;
}
