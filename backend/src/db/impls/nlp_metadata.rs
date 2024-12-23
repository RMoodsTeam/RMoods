use crate::db::db_error::DbError;
use crate::db::db_stored::DbStoredDependentlyInner;
use crate::db::from_db::FromDb;
use crate::db::model::DbNlpMetadata;
use crate::nlp::nlp_response::NlpMetadata;
use axum::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
impl DbStoredDependentlyInner for NlpMetadata {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, DbError> {
        let id = sqlx::query!(
            r#"
        INSERT INTO nlp_metadata (generated_in)
        VALUES ($1)
        RETURNING id as "id: Uuid";
        "#,
            self.generated_in
        )
        .fetch_one(&mut **tx)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for NlpMetadata {
    type DbModel = DbNlpMetadata;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, DbError> {
        Ok(NlpMetadata {
            generated_in: model.generated_in,
        })
    }
}
