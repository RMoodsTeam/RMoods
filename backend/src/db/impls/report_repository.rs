use crate::auth::google::GoogleId;
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
    pub user_name_pattern: Option<GoogleId>,
    pub contained_analysis_kinds: Option<Vec<NlpAnalysisKind>>,
    pub date_range: Option<DateRange>,
    pub title_pattern: Option<String>,
}

#[derive(Debug)]
pub(super) struct ReportQueryWhereClauses {
    user_id: String,
    contained_analysis_kinds: String,
    date_range: String,
    title_pattern: String,
}

impl ReportQuery {
    pub(super) fn into_where_clauses(self) -> ReportQueryWhereClauses {
        let noop = "1=1".to_string();

        let user_id_clause = self
            .user_name_pattern
            .and_then(|id| Some(format!("user_id = {}", id)))
            .unwrap_or_else(|| noop.clone());

        let contained_analysis_kinds_clause = match self.contained_analysis_kinds {
            Some(kinds) if kinds.is_empty() => noop.clone(),
            Some(kinds) => {
                let kind_clauses = kinds
                    .iter()
                    .map(|kind| format!("analyses_map.{}_id IS NOT NULL", kind.to_snake_case()))
                    .collect::<Vec<_>>()
                    .join(" AND ");
                kind_clauses
            }
            None => noop.clone(),
        };

        let date_range_clause = self
            .date_range
            .and_then(|range| {
                Some(format!(
                    "created_at >= {} AND created_at <= {}",
                    range.start_date, range.end_date
                ))
            })
            .unwrap_or_else(|| noop.clone());

        let title_pattern_clause = self
            .title_pattern
            .and_then(|pattern| Some(format!("title LIKE '%{}%'", pattern)))
            .unwrap_or_else(|| noop);

        ReportQueryWhereClauses {
            user_id: user_id_clause,
            contained_analysis_kinds: contained_analysis_kinds_clause,
            date_range: date_range_clause,
            title_pattern: title_pattern_clause,
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

        let query_str = format!(
            r#"
        SELECT * FROM reports r
        JOIN users u ON r.user_id = u.id
        JOIN report_metadata rm ON r.metadata_id = rm.id
        JOIN report_analyses_maps maps ON r.analyses_map_id = maps.id
        WHERE 
        u.name LIKE '%$1%'
        AND rm.report_created_at >= $2 AND rm.report_created_at <= $3
        AND {}
        AND r.title LIKE '%$4%'
        LIMIT $5 OFFSET $6
        "#,
            query
                .contained_analysis_kinds
                .map(|kinds| {
                    kinds
                        .iter()
                        .map(|kind| format!("maps.{}_id IS NOT NULL", kind.to_snake_case()))
                        .collect::<Vec<_>>()
                        .join(" AND ")
                })
                .unwrap_or("1=1".to_string())
        );

        let date_range = query.date_range.unwrap_or(DateRange {
            start_date: NaiveDateTime::UNIX_EPOCH.and_utc(),
            end_date: Utc::now(),
        });

        let query: Vec<DbReport> = sqlx::query_as(&query_str)
            .bind(query.user_name_pattern.unwrap_or(GoogleId::from("")))
            .bind(date_range.start_date)
            .bind(date_range.end_date)
            .bind(query.title_pattern.unwrap_or("".to_string()))
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

    #[test]
    fn test_into_where_clauses_user_id() {
        let query = ReportQuery {
            user_name_pattern: Some(GoogleId::from("test_user")),
            contained_analysis_kinds: None,
            date_range: None,
            title_pattern: None,
        };
        let where_clauses = query.into_where_clauses();

        assert_eq!(where_clauses.user_id, "user_id = test_user");
    }

    #[test]
    fn test_into_where_clauses_contained_analysis_kinds_one() {
        let query = ReportQuery {
            user_name_pattern: None,
            contained_analysis_kinds: Some(vec![NlpAnalysisKind::Sentiment]),
            date_range: None,
            title_pattern: None,
        };
        let where_clauses = query.into_where_clauses();

        assert_eq!(
            where_clauses.contained_analysis_kinds,
            "analyses_map.sentiment_id IS NOT NULL"
        );
    }

    #[test]
    fn test_into_where_clauses_contained_analysis_kinds_multiple() {
        let query = ReportQuery {
            user_name_pattern: None,
            contained_analysis_kinds: Some(vec![
                NlpAnalysisKind::Sentiment,
                NlpAnalysisKind::Clickbait,
                NlpAnalysisKind::HateSpeech,
            ]),
            date_range: None,
            title_pattern: None,
        };
        let where_clauses = query.into_where_clauses();

        assert_eq!(
            where_clauses.contained_analysis_kinds,
            "analyses_map.sentiment_id IS NOT NULL AND analyses_map.clickbait_id IS NOT NULL AND analyses_map.hate_speech_id IS NOT NULL"
        );
    }

    #[test]
    fn test_into_where_clauses_date_range() {
        let now = Utc::now();
        let query = ReportQuery {
            user_name_pattern: None,
            contained_analysis_kinds: None,
            date_range: Some(DateRange {
                start_date: now,
                end_date: now,
            }),
            title_pattern: None,
        };
        let where_clauses = query.into_where_clauses();

        assert_eq!(
            where_clauses.date_range,
            format!("created_at >= {} AND created_at <= {}", now, now)
        );
    }

    #[test]
    fn test_into_where_clauses_title_pattern() {
        let query = ReportQuery {
            user_name_pattern: None,
            contained_analysis_kinds: None,
            date_range: None,
            title_pattern: Some("test".to_string()),
        };
        let where_clauses = query.into_where_clauses();

        assert_eq!(where_clauses.title_pattern, "title LIKE '%test%'");
    }

    #[sqlx::test]
    async fn test_get_by_query_all_none() {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is set");
        let db = DbClient::new(sqlx::PgPool::connect(&db_url).await.unwrap());

        let reports_res = Report::get_by_query(
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

        let reports_res = Report::get_by_query(
            ReportQuery {
                user_name_pattern: Some(GoogleId::from("test_user")),
                contained_analysis_kinds: Some(vec![NlpAnalysisKind::Sentiment]),
                date_range: Some(DateRange {
                    start_date: Utc::now() - chrono::Duration::days(1),
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
