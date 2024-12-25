use crate::db::db_error::DbError;
use crate::db::db_stored::{DbStoredDependentlyInner, DbStoredInner};
use crate::db::from_db::FromDb;
use crate::db::model::{DbReport, DbReportAnalysesMap, DbReportMetadata};
use crate::db::pagination::DbPagination;
use crate::report::report::{Report, ReportAnalysesMap, ReportMetadata};
use crate::report::report_status::ReportStatus;
use axum::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

#[async_trait]
impl DbStoredInner for Report {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
        let metadata_uuid = self.metadata.inner_save(tx).await?;
        let analyses_map_uuid = self.analyses_map.inner_save(tx).await?;
        let user_id = sqlx::query!(
            r#"
            SELECT google_id as "id: String" FROM users WHERE google_id = $1
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
            user_id,
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
    /// Updates the report in the database.
    ///
    /// This method updates the report's title, description, public status, and status.
    /// The metadata and analyses map are not updated, as they are not expected to change.
    async fn inner_update(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE reports
            SET title = $1, description = $2, is_public = $3,
            is_successful = $4, is_in_progress = $5, is_error = $6, error_message = $7
            WHERE display_id = $8
            "#,
            self.title,
            self.description,
            self.is_public,
            self.status.is_successful(),
            self.status.is_in_progress(),
            self.status.is_error(),
            self.status.error_message(),
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

    async fn inner_delete_by_id(id: &str, pg_pool: &PgPool) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE display_id = $1
            "#,
            id
        )
        .execute(pg_pool)
        .await?;
        Ok(())
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
    use crate::db::db_stored::DbStored;
    use crate::db::impls::test_util::{get_db, get_test_report, get_test_user};
    use crate::report::report::Report;
    use serial_test::serial;

    #[sqlx::test]
    #[serial]
    async fn test_report_save() {
        let db = get_db().await;

        let user = get_test_user();
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id.clone());
        report.save(&db).await.unwrap();

        user.delete(&db).await.unwrap()
    }

    #[sqlx::test]
    #[serial]
    async fn test_report_get_by_id() {
        let db = get_db().await;

        let user = get_test_user();
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id);
        report.save(&db).await.unwrap();

        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap().unwrap();
        assert_eq!(fetched_report, report);

        fetched_report.delete(&db).await.unwrap();
        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap();
        assert_eq!(fetched_report, None);
    }

    #[sqlx::test]
    #[serial]
    async fn test_report_update() {
        let db = get_db().await;

        let user = get_test_user();
        user.save(&db).await.unwrap();

        let mut report = get_test_report("Test Report".to_string(), user.id);
        report.save(&db).await.unwrap();

        report.title = "Updated Title".to_string();
        report.description = "Updated Description".to_string();
        report.is_public = true;
        report.status = crate::report::report_status::ReportStatus::Error("Test Error".to_string());

        report.update(&db).await.unwrap();
        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap().unwrap();
        assert_eq!(fetched_report, report);

        report.delete(&db).await.unwrap();
        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap();
        assert_eq!(fetched_report, None);
    }

    #[sqlx::test]
    #[serial]
    async fn test_report_delete() {
        let db = get_db().await;

        let user = get_test_user();
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id);
        report.save(&db).await.unwrap();

        report.delete(&db).await.unwrap();

        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap();

        assert!(fetched_report.is_none());
    }
}
