use crate::db::db_client::DbClient;
use crate::db::db_error::DbError;
use crate::db::from_db::FromDb;
use crate::db::model::DbReport;
use crate::db::pagination::DbPagination;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::report::report::Report;
use axum::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

/// Represents a date range for querying reports.
///
/// Not validated for correctness, so the start date can be after the end date, later the query will just return no results.
/// This should be validated on the frontend anyway.
#[derive(Debug, Clone, Deserialize)]
pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Parameters for querying [Report]s
///
/// All fields are optional, so the query can be as specific or as general as needed.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportQuery {
    /// Filter by the user's name.
    ///
    /// The query will return reports where the user's name contains this string. It's used as `LIKE %<pattern>%`.
    #[serde(rename = "username")]
    pub user_name_pattern: Option<String>,
    /// Filter by the kinds of analyses contained in the report.
    ///
    /// The query will return reports that contain all the specified kinds of analyses.
    #[serde(rename = "analyses")]
    #[serde(default)]
    pub contained_analysis_kinds: Vec<NlpAnalysisKind>,
    /// Filter by the creation date of the report.
    ///
    /// Filters out reports created before this date, passed as a UNIX timestamp.
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_dt")]
    pub start_date: Option<DateTime<Utc>>,
    /// Filter by the creation date of the report.
    ///
    /// Filters out reports created after this date, passed as a UNIX timestamp.
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_dt")]
    pub end_date: Option<DateTime<Utc>>,
    /// Filter by the title of the report.
    ///
    /// The query will return reports where the title contains this string. It's used as `LIKE %<pattern>%`.
    #[serde(rename = "title")]
    pub title_pattern: Option<String>,
    /// Pagination parameters for the query.
    ///
    /// If not provided, the default values are used: page 1, 30 items per page.
    #[serde(flatten)]
    pub pagination: DbPagination,
}

impl Default for ReportQuery {
    fn default() -> Self {
        Self {
            user_name_pattern: None,
            contained_analysis_kinds: vec![],
            start_date: None,
            end_date: None,
            title_pattern: None,
            pagination: DbPagination::default(),
        }
    }
}

/// Deserializes an optional UNIX timestamp into an optional [DateTime<Utc>].
fn deserialize_optional_dt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<i64>::deserialize(deserializer)?
        .map(|secs| {
            DateTime::from_timestamp(secs, 0)
                .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))
        })
        .transpose()
}

/// Arguments for the query to get reports.
///
/// Created by converting a [ReportQuery] to this struct.
/// Values in this struct are meant as bind parameters for the SQL query.
/// Depending on the query, some values might be empty or have a default value, which will still maintain the query's correctness.
struct ReportQueryBindArgs {
    /// The user's name pattern.
    ///
    /// If not provided in the query, it's an empty string. It's used as `LIKE %<pattern>%`.
    user_name_pattern: String,
    /// The concatenated `WHERE` clauses for checking if the report contains the specified kinds of analyses.
    ///
    /// If not provided in the query, it's `1=1`, which is always true and maintains the query's correctness.
    contained_analysis_kinds_clauses: String,
    /// The start date of the date range.
    ///
    /// If not provided in the query, it's the UNIX epoch.
    start_date: DateTime<Utc>,
    /// The end date of the date range.
    ///
    /// If not provided in the query, it's the current date and time.
    end_date: DateTime<Utc>,
    /// The title pattern.
    ///
    /// If not provided in the query, it's an empty string. It's used as `LIKE %<pattern>%`.
    title_pattern: String,
}

impl ReportQuery {
    /// Converts the query to bind parameters for the SQL query.
    ///
    /// Depending on the query parameters, this method computes the correct values for the bind parameters.
    /// If a parameter is not provided, it's replaced with a default value that maintains the query's syntactic correctness, but has no effect on the results.
    ///
    /// For example, if the username pattern is not provided, it's an empty string, which will match any username.
    /// Another example: the date range, where if it's not provided, the start date is the UNIX epoch and the end date is the current date and time, which will match any report.
    fn into_bind_args(self) -> ReportQueryBindArgs {
        let user_name_pattern = self.user_name_pattern.unwrap_or(String::new());
        let contained_analysis_kinds_clauses = if self.contained_analysis_kinds.is_empty() {
            "1=1".to_string()
        } else {
            self.contained_analysis_kinds
                .into_iter()
                .map(|kind| {
                    format!(
                        "report_analyses_maps.{}_id IS NOT NULL",
                        kind.to_snake_case()
                    )
                })
                .collect::<Vec<_>>()
                .join(" AND ")
        };

        let start_date = self
            .start_date
            .unwrap_or(NaiveDateTime::UNIX_EPOCH.and_utc());
        let end_date = self.end_date.unwrap_or(Utc::now());
        let title_pattern = self.title_pattern.unwrap_or("".to_string());

        ReportQueryBindArgs {
            user_name_pattern,
            contained_analysis_kinds_clauses,
            start_date,
            end_date,
            title_pattern,
        }
    }
}

/// Repository trait for reports.
///
/// Specifies methods for fetching reports from the database.
#[async_trait]
pub trait ReportRepository: Sized {
    async fn get_by_query(query: ReportQuery, db: &DbClient) -> Result<Vec<Self>, DbError>;
}

#[async_trait]
impl ReportRepository for Report {
    /// Fetches reports from the database based on the query and pagination.
    ///
    /// # WARNING
    /// This function dynamically creates an SQL query based on parameters provided **by the user**.
    /// However, it's carefully checked and validated to prevent SQL injection.
    /// Most parameters are used as bind parameters, so they are not directly interpolated into the query string.
    ///
    /// The only part where we really construct the query string dynamically is the `WHERE` clause for the contained analysis kinds.
    /// This is done by concatenating the clauses for each kind of analysis that the user wants to filter by.
    /// It's safe, because the kinds are passed as a [Vec] of [NlpAnalysisKind]s, which are validated by `serde` deserialization.
    ///
    /// **Any changes to this function need to be carefully reviewed to prevent SQL injection.**
    async fn get_by_query(query: ReportQuery, db: &DbClient) -> Result<Vec<Self>, DbError> {
        let (limit, offset) = query.pagination.clone().into_limit_and_offset();

        let bind_args = query.into_bind_args();

        let query_str = format!(
            r#"
        SELECT * FROM reports r
        JOIN users u ON r.user_id = u.google_id
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
            contained_analysis_kinds: vec![NlpAnalysisKind::Sentiment],
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
            contained_analysis_kinds: vec![
                NlpAnalysisKind::Sentiment,
                NlpAnalysisKind::Clickbait,
                NlpAnalysisKind::HateSpeech,
            ],
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
            start_date: Some(now - chrono::Duration::days(1)),
            end_date: Some(now),
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
                contained_analysis_kinds: vec![],
                start_date: None,
                end_date: None,
                title_pattern: None,
                pagination: DbPagination::default(),
            },
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
                contained_analysis_kinds: vec![
                    NlpAnalysisKind::Sentiment,
                    NlpAnalysisKind::HateSpeech,
                ],
                start_date: Some(Utc::now() - chrono::Duration::days(3)),
                end_date: Some(Utc::now()),
                title_pattern: Some("test".to_string()),
                pagination: DbPagination::new(0, 10),
            },
            &db,
        )
        .await
        .unwrap();
    }
}
