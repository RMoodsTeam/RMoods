use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::{NlpAnalysis, NlpMetadata};
use axum::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

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
    async fn save(&self, db: &PgPool) -> Result<Uuid, sqlx::Error>;
}

#[async_trait]
impl DbStoredDependently for NlpMetadata {
    async fn save(&self, db: &PgPool) -> Result<Uuid, Error> {
        let id = sqlx::query!(
            r#"
            INSERT INTO nlp_metadata (generated_in)
            VALUES ($1)
            RETURNING id as "id: Uuid";
            "#,
            self.generated_in
        )
        .fetch_one(db)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl DbStoredDependently for NlpAnalysis {
    async fn save(&self, db: &PgPool) -> Result<Uuid, Error> {
        let metadata_uuid = self.metadata.save(db).await?;
        let id = sqlx::query!(
            r#"
            INSERT INTO nlp_analysis (nlp_metadata_id, kind, analysis)
            VALUES ($1, $2, $3)
            RETURNING id as "id: Uuid";
            "#,
            metadata_uuid,
            &self.kind as &NlpAnalysisKind,
            serde_json::to_value(&self.results).unwrap()
        )
        .fetch_one(db)
        .await?
        .id;
        Ok(id)
    }
}
