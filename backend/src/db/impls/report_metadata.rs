use crate::db::db_error::DbError;
use crate::db::db_stored::DbStoredDependentlyInner;
use crate::db::from_db::FromDb;
use crate::db::model::DbReportMetadata;
use crate::report::report::ReportMetadata;
use axum::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
impl DbStoredDependentlyInner for ReportMetadata {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, DbError> {
        let id = sqlx::query!(
            r#"
            INSERT INTO report_metadata (report_created_at, report_updated_at)
            VALUES ($1, $2)
            RETURNING id as "id: Uuid";
            "#,
            self.created_at,
            self.updated_at
        )
        .fetch_one(&mut **tx)
        .await?
        .id;
        Ok(id)
    }
}

#[async_trait]
impl FromDb for ReportMetadata {
    type DbModel = DbReportMetadata;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, DbError> {
        Ok(ReportMetadata {
            created_at: model.report_created_at,
            updated_at: model.report_updated_at,
        })
    }
}
