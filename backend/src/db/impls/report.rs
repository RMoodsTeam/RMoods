use crate::db::db_stored::{DbStoredDependentlyInner, DbStoredInner};
use crate::db::from_db::FromDb;
use crate::db::model::{DbReport, DbReportAnalysesMap, DbReportMetadata};
use crate::db::pagination::DbPagination;
use crate::nlp::report::{Report, ReportAnalysesMap, ReportMetadata};
use axum::async_trait;
use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
impl DbStoredInner for Report {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<(), Error> {
        let metadata_uuid = self.metadata.inner_save(tx).await?;
        let analyses_map_uuid = self.analyses_map.inner_save(tx).await?;
        let user_uuid = sqlx::query!(
            r#"
            SELECT id as "id: Uuid" FROM users WHERE google_sub = $1
            "#,
            self.user_id
        )
        .fetch_one(&mut **tx)
        .await?
        .id;

        sqlx::query!(
            r#"
            INSERT INTO reports (
            display_id, user_id, title, description, is_public, metadata_id, analyses_map_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            self.id,
            user_uuid,
            self.title,
            self.description,
            self.is_public,
            metadata_uuid,
            analyses_map_uuid
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
    async fn inner_update(&self, tx: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE reports
            SET title = $1, description = $2, is_public = $3
            WHERE display_id = $4
            "#,
            self.title,
            self.description,
            self.is_public,
            self.id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
    async fn inner_delete(&self, tx: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE display_id = $1
            "#,
            self.id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn inner_get_by_id(
        id: &str,
        tx: &mut Transaction<Postgres>,
    ) -> Result<Option<Self>, Error> {
        let report = sqlx::query_as!(
            DbReport,
            r#"
            SELECT *
            FROM reports
            WHERE display_id = $1
            "#,
            id
        )
        .fetch_optional(&mut **tx)
        .await?;

        let report = match report {
            Some(report) => report,
            None => return Ok(None),
        };

        let metadata = {
            let db_metadata = sqlx::query_as!(
                DbReportMetadata,
                r#"
            SELECT *
            FROM report_metadata
            WHERE id = $1
            "#,
                report.metadata_id
            )
            .fetch_one(&mut **tx)
            .await?;

            ReportMetadata::from_db_model(db_metadata, tx).await?
        };

        let analyses_map = {
            let map = sqlx::query_as!(
                DbReportAnalysesMap,
                r#"
            SELECT *
            FROM report_analyses_maps
            WHERE id = $1
            "#,
                report.analyses_map_id
            )
            .fetch_one(&mut **tx)
            .await?;

            ReportAnalysesMap::from_db_model(map, tx).await?
        };

        Ok(Some(Report {
            id: report.display_id,
            user_id: report.user_id,
            title: report.title,
            description: report.description,
            is_public: report.is_public,
            metadata,
            analyses_map,
        }))
    }
    async fn inner_get_all(
        pagination: DbPagination,
        tx: &mut Transaction<Postgres>,
    ) -> Result<Vec<Self>, Error> {
        unimplemented!()
    }
}
