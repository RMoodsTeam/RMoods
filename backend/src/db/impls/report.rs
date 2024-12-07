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
    async fn inner_save(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        let metadata_uuid = self.metadata.inner_save(db).await?;
        let analyses_uuid = self.analyses_map.inner_save(db).await?;
        let user_uuid = sqlx::query!(
            r#"
            SELECT id as "id: Uuid" FROM users WHERE google_sub = $1
            "#,
            self.user_id
        )
        .fetch_one(&mut **db)
        .await?
        .id;

        sqlx::query!(
            r#"
            INSERT INTO reports (
            user_id, title, description, is_public, metadata_id, analyses_map_id
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_uuid,
            self.title,
            self.description,
            self.is_public,
            metadata_uuid,
            analyses_uuid
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }
    async fn inner_update(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
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
        .execute(&mut **db)
        .await?;
        Ok(())
    }
    async fn inner_delete(&self, db: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE display_id = $1
            "#,
            self.id
        )
        .execute(&mut **db)
        .await?;
        Ok(())
    }

    async fn inner_get_by_id(
        id: &str,
        db: &mut Transaction<Postgres>,
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
        .fetch_optional(&mut **db)
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
            .fetch_one(&mut **db)
            .await?;

            ReportMetadata::from_db_model(db_metadata, db).await?
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
            .fetch_one(&mut **db)
            .await?;

            ReportAnalysesMap::from_db_model(map, db).await?
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
        db: &mut Transaction<Postgres>,
    ) -> Result<Vec<Self>, Error> {
        todo!()
    }
}
