use crate::db::db_stored::{DbStoredDependentlyInner, DbStoredInner};
use crate::db::from_db::FromDb;
use crate::db::model::{DbReport, DbReportAnalysesMap, DbReportMetadata};
use crate::db::pagination::DbPagination;
use crate::nlp::report::{Report, ReportAnalysesMap, ReportMetadata};
use axum::async_trait;
use sqlx::{Error, PgPool, Postgres, Transaction};
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

    async fn inner_get_by_id(id: &str, pool: &PgPool) -> Result<Option<Self>, Error> {
        let db_report = sqlx::query_as!(
            DbReport,
            r#"
            SELECT *
            FROM reports
            WHERE display_id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        let report = match db_report {
            Some(res) => res,
            None => return Ok(None),
        };

        let report = Report::from_db_model(report, pool).await?;

        Ok(Some(report))
    }
    async fn inner_get_all(pagination: DbPagination, pool: &PgPool) -> Result<Vec<Self>, Error> {
        unimplemented!()
    }
}

#[async_trait]
impl FromDb for Report {
    type DbModel = DbReport;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, Error> {
        let metadata = {
            let db_metadata = sqlx::query_as!(
                DbReportMetadata,
                r#"
            SELECT *
            FROM report_metadata
            WHERE id = $1
            "#,
                model.metadata_id
            )
            .fetch_one(pool)
            .await?;

            ReportMetadata::from_db_model(db_metadata, pool).await?
        };

        let analyses_map = {
            let map = sqlx::query_as!(
                DbReportAnalysesMap,
                r#"
            SELECT *
            FROM report_analyses_maps
            WHERE id = $1
            "#,
                model.analyses_map_id
            )
            .fetch_one(pool)
            .await?;

            ReportAnalysesMap::from_db_model(map, pool).await?
        };

        Ok(Report {
            id: model.display_id,
            user_id: model.user_id,
            title: model.title,
            description: model.description,
            is_public: model.is_public,
            metadata,
            analyses_map,
        })
    }
}
