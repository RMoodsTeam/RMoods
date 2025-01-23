use crate::db::db_error::DbError;
use crate::db::db_stored::{DbStoredDependentlyInner, DbStoredInner};
use crate::db::from_db::FromDb;
use crate::db::model::{DbDataRequest, DbNlpAnalysis, DbReport};
use crate::db::pagination::DbPagination;
use crate::fetcher::data_request::FetcherDataRequest;
use crate::nlp::nlp_analysis::NlpAnalysis;
use crate::report::report::Report;
use crate::report::report_status::ReportStatus;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;

impl DbStoredInner for Report {
    async fn inner_save(&self, tx: &mut Transaction<'_, Postgres>) -> Result<(), DbError> {
        let user_id = sqlx::query!(
            r#"
            SELECT google_id as "id: String" FROM users WHERE google_id = $1
            "#,
            self.user_id
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|_| {
            DbError::NotFound(format!(
                "User with id {} not found. Cannot create a report assigned to them.",
                self.user_id
            ))
        })?
        .id;

        let report_id = sqlx::query!(
            r#"
            INSERT INTO reports (
            id, user_id, title, description, is_public,
            is_successful, is_in_progress, is_error, error_message,
            created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7 , $8, $9, $10, $11)
            RETURNING id;
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
            self.created_at,
            self.updated_at
        )
        .fetch_one(&mut **tx)
        .await?
        .id;

        for (_, analysis) in &self.analyses {
            analysis.inner_save(report_id.as_str(), tx).await?;
        }

        self.data_request.inner_save(report_id.as_str(), tx).await?;

        Ok(())
    }
    /// Updates the report in the database.
    ///
    /// This method updates the report's title, description, public status, and status.
    /// The metadata and analyses map are not updated, as they are not expected to change.
    async fn inner_update(&self, tx: &mut Transaction<'_, Postgres>) -> Result<(), DbError> {
        for (_, analysis) in &self.analyses {
            analysis.inner_save(self.id.as_str(), tx).await?;
        }

        sqlx::query!(
            r#"
            UPDATE reports
            SET title = $1, description = $2, is_public = $3,
            is_successful = $4, is_in_progress = $5, is_error = $6, error_message = $7,
            created_at = $8
            WHERE id = $9
            "#,
            self.title,
            self.description,
            self.is_public,
            self.status.is_successful(),
            self.status.is_in_progress(),
            self.status.is_error(),
            self.status.error_message(),
            self.created_at,
            self.id,
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
    async fn inner_delete(&self, tx: &mut Transaction<'_, Postgres>) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE id = $1
            "#,
            self.id.to_string()
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn inner_get_by_id(id: &str, pool: &PgPool) -> Result<Option<Self>, DbError> {
        let db_report = sqlx::query_as!(
            DbReport,
            r#"
            SELECT *, 0 as total_reports
            FROM reports
            WHERE id = $1
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
            DELETE FROM reports WHERE id = $1
            "#,
            id
        )
        .execute(pg_pool)
        .await?;
        Ok(())
    }

    async fn inner_get_all(
        _pagination: DbPagination,
        _pool: &PgPool,
    ) -> Result<Vec<Self>, DbError> {
        unimplemented!()
    }
}

impl FromDb for Report {
    type DbModel = DbReport;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, DbError> {
        let status = ReportStatus::from_booleans(
            model.is_successful,
            model.is_in_progress,
            model.is_error,
            model.error_message,
        )?;

        let analyses_res = sqlx::query_as!(
            DbNlpAnalysis,
            r#"
            SELECT *
            FROM nlp_analyses
            WHERE report_id = $1
            "#,
            model.id
        )
        .fetch_all(pool)
        .await?;

        let analyses_futures = analyses_res
            .into_iter()
            .map(|analysis| async { NlpAnalysis::from_db_model(analysis, pool).await });

        let analyses_vec = futures::future::join_all(analyses_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<NlpAnalysis>, DbError>>()?;

        let analyses: HashMap<_, _> = analyses_vec
            .into_iter()
            .map(|analysis| (analysis.kind.clone(), analysis))
            .collect();

        let data_request_res = sqlx::query_as!(
            DbDataRequest,
            r#"
            SELECT *
            FROM data_requests
            WHERE report_id = $1
            "#,
            model.id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DbError::NotFound(format!(
                "Data request for report with id {} not found",
                model.id
            )),
            _ => DbError::from(e),
        })?;

        let data_request = FetcherDataRequest::from_db_model(data_request_res, pool).await?;

        Ok(Report {
            id: model.id,
            user_id: model.user_id,
            title: model.title,
            description: model.description,
            is_public: model.is_public,
            status,
            analyses,
            processed_analyses: None,
            data_request,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::db::db_client::DbClient;
    use crate::db::db_stored::DbStored;
    use crate::db::impls::test_util::{get_test_report, get_test_user};
    use crate::report::report::Report;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_report_save(pool: PgPool) {
        let db = DbClient::new(pool);

        let user = get_test_user("123".to_string());
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id.clone());
        report.save(&db).await.unwrap();

        user.delete(&db).await.unwrap()
    }

    #[sqlx::test]
    async fn test_report_get_by_id(pool: PgPool) {
        let db = DbClient::new(pool);

        let user = get_test_user("123".to_string());
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id.clone());
        report.save(&db).await.unwrap();

        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap().unwrap();
        assert_eq!(fetched_report, report);

        fetched_report.delete(&db).await.unwrap();
        user.delete(&db).await.unwrap();
    }

    #[sqlx::test]
    async fn test_report_update(pool: PgPool) {
        let db = DbClient::new(pool);

        let user = get_test_user("123".to_string());
        user.save(&db).await.unwrap();

        let mut report = get_test_report("Test Report".to_string(), user.id.clone());
        report.save(&db).await.unwrap();

        report.title = "Updated Title".to_string();
        report.description = "Updated Description".to_string();
        report.is_public = true;
        report.status = crate::report::report_status::ReportStatus::Error("Test Error".to_string());

        report.update(&db).await.unwrap();
        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap().unwrap();

        assert_eq!(fetched_report.title, report.title);
        assert_eq!(fetched_report.description, report.description);
        assert_eq!(fetched_report.is_public, report.is_public);
        assert_eq!(fetched_report.status, report.status);
        assert_eq!(fetched_report.created_at, report.created_at);
        assert_ne!(fetched_report.updated_at, report.updated_at); // updated_at should be updated

        report.delete(&db).await.unwrap();
        user.delete(&db).await.unwrap();
    }

    #[sqlx::test]
    async fn test_report_delete(pool: PgPool) {
        let db = DbClient::new(pool);

        let user = get_test_user("123".to_string());
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id);
        report.save(&db).await.unwrap();

        report.delete(&db).await.unwrap();

        let fetched_report = Report::get_by_id(&report.id, &db).await.unwrap();
        assert!(fetched_report.is_none());
    }

    #[sqlx::test]
    async fn test_report_analyses_saving_deleting(pool: PgPool) {
        let db = DbClient::new(pool);

        let user = get_test_user("123".to_string());
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id);
        report.save(&db).await.unwrap();

        let analyses_before = sqlx::query!(
            r#"
            SELECT * FROM nlp_analyses WHERE report_id = $1
            "#,
            report.id
        )
        .fetch_all(db.raw_db())
        .await
        .unwrap();
        assert!(!analyses_before.is_empty());

        report.delete(&db).await.unwrap();

        let analyses_after = sqlx::query!(
            r#"
            SELECT * FROM nlp_analyses WHERE report_id = $1
            "#,
            report.id
        )
        .fetch_all(db.raw_db())
        .await
        .unwrap();

        assert!(analyses_after.is_empty());
    }

    #[sqlx::test]
    async fn test_report_data_request_saving_deleting(pool: PgPool) {
        let db = DbClient::new(pool);

        let user = get_test_user("123".to_string());
        user.save(&db).await.unwrap();

        let report = get_test_report("Test Report".to_string(), user.id);
        report.save(&db).await.unwrap();

        let data_request_before = sqlx::query!(
            r#"
            SELECT * FROM data_requests WHERE report_id = $1
            "#,
            report.id
        )
        .fetch_all(db.raw_db())
        .await
        .unwrap();

        assert!(!data_request_before.is_empty());
    }
}
