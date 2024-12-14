use crate::db::db_stored::DbStoredDependentlyInner;
use crate::db::from_db::FromDb;
use crate::db::model::{DbNlpAnalysis, DbNlpMetadata};
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::nlp_response::{NlpAnalysis, NlpMetadata};
use axum::async_trait;
use sqlx::{Error, PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
impl DbStoredDependentlyInner for NlpAnalysis {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error> {
        let metadata_uuid = self.metadata.inner_save(tx).await?;
        let id = sqlx::query!(
            r#"
            INSERT INTO nlp_analyses (nlp_metadata_id, kind, analysis)
            VALUES ($1, $2, $3)
            RETURNING id as "id: Uuid";
            "#,
            metadata_uuid,
            &self.kind.to_snake_case(),
            serde_json::to_value(&self.results).unwrap() // TODO HANDLE ERROR
        )
        .fetch_one(&mut **tx)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for NlpAnalysis {
    type DbModel = DbNlpAnalysis;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, Error> {
        let nlp_metadata = sqlx::query_as!(
            DbNlpMetadata,
            r#"SELECT * FROM nlp_metadata WHERE id = $1"#,
            model.nlp_metadata_id
        )
        .fetch_one(pool)
        .await?;
        Ok(NlpAnalysis {
            kind: NlpAnalysisKind::from_snake_case(&model.kind).unwrap(),
            results: serde_json::from_value(model.analysis.clone()).unwrap(),
            metadata: NlpMetadata {
                generated_in: nlp_metadata.generated_in,
            },
        })
    }
}
