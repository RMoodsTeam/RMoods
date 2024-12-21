use crate::db::db_error::DbError;
use crate::db::db_stored::{DbStoredDependentlyInner, DbStoredInner};
use crate::db::from_db::FromDb;
use crate::db::model::{DbReport, DbReportAnalysesMap, DbReportMetadata};
use crate::db::pagination::DbPagination;
use crate::report::report::{Report, ReportAnalysesMap, ReportMetadata, ReportStatus};
use axum::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
impl DbStoredInner for Report {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
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
            display_id, user_id, title, description, is_public,
            is_successful, is_in_progress, is_error, error_message, 
            metadata_id, analyses_map_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7 , $8, $9, $10, $11)
            "#,
            self.id,
            user_uuid,
            self.title,
            self.description,
            self.is_public,
            self.status.is_successful(),
            self.status.is_in_progress(),
            self.status.is_error(),
            self.status.error_message(),
            metadata_uuid,
            analyses_map_uuid
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
    async fn inner_update(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
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
    async fn inner_delete(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
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

    async fn inner_get_by_id(id: &str, pool: &PgPool) -> Result<Option<Self>, DbError> {
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
    async fn inner_get_all(pagination: DbPagination, pool: &PgPool) -> Result<Vec<Self>, DbError> {
        unimplemented!()
    }
}

#[async_trait]
impl FromDb for Report {
    type DbModel = DbReport;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, DbError> {
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

        let status = ReportStatus::from_booleans(
            model.is_successful,
            model.is_in_progress,
            model.is_error,
            model.error_message,
        )?;

        Ok(Report {
            id: model.display_id,
            user_id: model.user_id,
            title: model.title,
            description: model.description,
            is_public: model.is_public,
            status,
            metadata,
            analyses_map,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::db::db_client::DbClient;
    use crate::db::db_stored::DbStored;
    use crate::db::impls::test_util::insert_test_user;

    #[sqlx::test]
    async fn test_report_save() {
        use crate::report::report::{Report, ReportAnalysesMap, ReportMetadata, ReportStatus};
        use sqlx::postgres::PgPoolOptions;
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(std::env!("DATABASE_URL"))
            .await
            .unwrap();
        let db = DbClient::new(pool);

        let user_id = insert_test_user(&db.raw_db()).await.unwrap();

        let report = Report {
            id: nanoid::nanoid!(),
            user_id,
            title: "Test Report".to_string(),
            description: "Test Description".to_string(),
            is_public: false,
            status: ReportStatus::InProgress,
            metadata: ReportMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            analyses_map: ReportAnalysesMap {
                analyses: Default::default(),
            },
        };

        report.save(&db).await.unwrap();
    }
}
