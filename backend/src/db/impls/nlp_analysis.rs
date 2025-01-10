use crate::db::db_error::DbError;
use crate::db::db_stored::DbStoredDependentlyInner;
use crate::db::from_db::FromDb;
use crate::db::model::DbNlpAnalysis;
use crate::nlp::analysis_kind::NlpAnalysisKind;
use crate::nlp::nlp_response::NlpAnalysis;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

impl DbStoredDependentlyInner for NlpAnalysis {
    async fn inner_save(
        &self,
        parent_id: &str,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Uuid, DbError> {
        let analysis_id = sqlx::query!(
            r#"
            INSERT INTO nlp_analyses (report_id, kind, generated_in, analysis)
            VALUES ($1, $2, $3, $4)
            RETURNING id as "id: Uuid";
            "#,
            parent_id,
            &self.kind.to_snake_case(),
            self.generated_in,
            serde_json::to_value(&self.results).unwrap() // TODO HANDLE ERROR
        )
        .fetch_one(&mut **tx)
        .await?
        .id;

        Ok(analysis_id)
    }
}

impl FromDb for NlpAnalysis {
    type DbModel = DbNlpAnalysis;
    async fn from_db_model(model: Self::DbModel, _pool: &PgPool) -> Result<Self, DbError> {
        Ok(NlpAnalysis {
            kind: NlpAnalysisKind::from_snake_case(&model.kind).unwrap(),
            results: serde_json::from_value(model.analysis.clone()).unwrap(),
            generated_in: model.generated_in,
        })
    }
}
