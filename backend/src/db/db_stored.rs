use crate::nlp::nlp_response::NlpMetadata;
use axum::async_trait;
use sqlx::{Error, PgPool};

/// Implemented for structs that can be saved to our database
#[async_trait]
pub trait DbStored {
    type DbStruct;
    async fn save(&self, db: &PgPool) -> Result<(), sqlx::Error>;
    async fn delete(&self, db: &PgPool) -> Result<(), sqlx::Error>;
    async fn update(&self, db: &PgPool) -> Result<(), sqlx::Error>;
}

#[async_trait]
pub trait DbStoredDependently {
    async fn save(&self, db: &PgPool) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl DbStoredDependently for NlpMetadata {
    async fn save(&self, db: &PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO nlp_metadata (generated_in)
            VALUES ($1)
            "#,
            self.generated_in
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
