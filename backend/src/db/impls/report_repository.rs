use crate::db::db_client::DbClient;
use crate::db::from_db::FromDb;
use crate::db::model::DbReport;
use crate::db::pagination::DbPagination;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::report::Report;
use axum::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::Error;

#[derive(Debug, Clone)]
pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ReportQuery {
    pub user_name_pattern: Option<String>,
    pub contained_analysis_kinds: Option<Vec<NlpAnalysisKind>>,
    pub date_range: Option<DateRange>,
    pub title_pattern: Option<String>,
}

impl Default for ReportQuery {
    fn default() -> Self {
        Self {
            user_name_pattern: None,
            contained_analysis_kinds: None,
            date_range: None,
            title_pattern: None,
        }
    }
}

struct ReportQueryBindArgs {
    user_name_pattern: String,
    contained_analysis_kinds_clauses: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    title_pattern: String,
}

impl ReportQuery {
    fn into_bind_args(self) -> ReportQueryBindArgs {
        let user_name_pattern = self.user_name_pattern.unwrap_or(String::new());
        let contained_analysis_kinds_clauses = self
            .contained_analysis_kinds
            .map(|kinds| {
                kinds
                    .iter()
                    .map(|kind| {
                        format!(
                            "report_analyses_maps.{}_id IS NOT NULL",
                            kind.to_snake_case()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" AND ")
            })
            .unwrap_or("1=1".to_string());
        let date_range = self.date_range.unwrap_or(DateRange {
            start_date: NaiveDateTime::UNIX_EPOCH.and_utc(),
            end_date: Utc::now(),
        });
        let title_pattern = self.title_pattern.unwrap_or("".to_string());

        ReportQueryBindArgs {
            user_name_pattern,
            contained_analysis_kinds_clauses,
            start_date: date_range.start_date,
            end_date: date_range.end_date,
            title_pattern,
        }
    }
}

#[async_trait]
pub trait ReportRepository: Sized {
    async fn get_by_query(
        query: ReportQuery,
        pagination: DbPagination,
        db: &DbClient,
    ) -> Result<Vec<Self>, Error>;
}

#[async_trait]
impl ReportRepository for Report {
    async fn get_by_query(
        query: ReportQuery,
        pagination: DbPagination,
        db: &DbClient,
    ) -> Result<Vec<Self>, Error> {
        let (limit, offset) = pagination.into_limit_and_offset();

        let bind_args = query.into_bind_args();

        let query_str = format!(
            r#"
        SELECT * FROM reports r
        JOIN users u ON r.user_id = u.id
        JOIN report_metadata rm ON r.metadata_id = rm.id
        JOIN report_analyses_maps ON r.analyses_map_id = report_analyses_maps.id
        WHERE 
        u.name LIKE '%$1%'
        AND rm.report_created_at >= $2 AND rm.report_created_at <= $3
        AND {}
        AND r.title LIKE '%$4%'
        LIMIT $5 OFFSET $6
        "#,
            bind_args.contained_analysis_kinds_clauses
        );

        let query: Vec<DbReport> = sqlx::query_as(&query_str)
            .bind(bind_args.user_name_pattern)
            .bind(bind_args.start_date)
            .bind(bind_args.end_date)
            .bind(bind_args.title_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(db.raw_db())
            .await?;

        let reports_fut = query
            .into_iter()
            .map(|db_report| async { Report::from_db_model(db_report, db.raw_db()).await });
        let reports = futures::future::join_all(reports_fut)
            .await
            .into_iter()
            .collect::<Result<_, _>>()?;

        Ok(reports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn test_into_where_clauses_user_id() {
        let query = ReportQuery {
            user_name_pattern: Some(String::from("test_user")),
            ..Default::default()
        };
        let args = query.into_bind_args();

        assert_eq!(args.user_name_pattern, "test_user");
    }

    #[test]
    fn test_into_where_clauses_user_id_empty() {
        let query = ReportQuery::default();
        let args = query.into_bind_args();

        assert_eq!(args.user_name_pattern, "");
    }

    #[test]
    fn test_into_where_clauses_contained_analysis_kinds_one() {
        let query = ReportQuery {
            contained_analysis_kinds: Some(vec![NlpAnalysisKind::Sentiment]),
            ..Default::default()
        };
        let args = query.into_bind_args();

        assert_eq!(
            args.contained_analysis_kinds_clauses,
            "report_analyses_maps.sentiment_id IS NOT NULL"
        );
    }

    #[test]
    fn test_into_where_clauses_contained_analysis_kinds_multiple() {
        let query = ReportQuery {
            contained_analysis_kinds: Some(vec![
                NlpAnalysisKind::Sentiment,
                NlpAnalysisKind::Clickbait,
                NlpAnalysisKind::HateSpeech,
            ]),
            ..Default::default()
        };
        let args = query.into_bind_args();

        assert_eq!(
            args.contained_analysis_kinds_clauses,
            "report_analyses_maps.sentiment_id IS NOT NULL AND report_analyses_maps.clickbait_id IS NOT NULL AND report_analyses_maps.hate_speech_id IS NOT NULL"
        );
    }

    #[test]
    fn test_into_where_clauses_date_range() {
        let now = Utc::now();
        let query = ReportQuery {
            date_range: Some(DateRange {
                start_date: now - chrono::Duration::days(1),
                end_date: now,
            }),
            ..Default::default()
        };
        let args = query.into_bind_args();

        assert_eq!(args.start_date, now - chrono::Duration::days(1));
        assert_eq!(args.end_date, now);
    }

    #[test]
    fn test_into_where_clauses_date_range_empty() {
        let now = Utc::now();
        let query = ReportQuery::default();
        let args = query.into_bind_args();

        assert_eq!(args.start_date, NaiveDateTime::UNIX_EPOCH.and_utc());
        // We need to check the time as well, as the Utc::now() call in the test and the one in the function might be a few milliseconds apart
        // This is why we check if the difference is less than a second
        assert!(args.end_date - now < chrono::Duration::seconds(1));
    }

    #[test]
    fn test_into_where_clauses_title_pattern() {
        let query = ReportQuery {
            title_pattern: Some("test".to_string()),
            ..Default::default()
        };
        let args = query.into_bind_args();

        assert_eq!(args.title_pattern, "test");
    }

    #[test]
    fn test_into_where_clauses_title_pattern_empty() {
        let query = ReportQuery::default();
        let args = query.into_bind_args();

        assert_eq!(args.title_pattern, "");
    }

    #[sqlx::test]
    async fn test_get_by_query_all_none() {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is set");
        let db = DbClient::new(sqlx::PgPool::connect(&db_url).await.unwrap());

        let _ = Report::get_by_query(
            ReportQuery {
                user_name_pattern: None,
                contained_analysis_kinds: None,
                date_range: None,
                title_pattern: None,
            },
            DbPagination::new(0, 10),
            &db,
        )
        .await
        .unwrap();
    }

    #[sqlx::test]
    async fn test_get_by_query_all_filled() {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let db = DbClient::new(sqlx::PgPool::connect(&db_url).await.unwrap());

        let _ = Report::get_by_query(
            ReportQuery {
                user_name_pattern: Some(String::from("test_user")),
                contained_analysis_kinds: Some(vec![
                    NlpAnalysisKind::Sentiment,
                    NlpAnalysisKind::HateSpeech,
                ]),
                date_range: Some(DateRange {
                    start_date: Utc::now() - chrono::Duration::days(3),
                    end_date: Utc::now(),
                }),
                title_pattern: Some("test".to_string()),
            },
            DbPagination::new(0, 10),
            &db,
        )
        .await
        .unwrap();
    }
}
