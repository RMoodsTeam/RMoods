use crate::auth::user::GoogleId;
use crate::db::db_client::DbClient;
use crate::db::db_error::DbError;
use crate::db::from_db::FromDb;
use crate::db::model::DbReport;
use crate::db::pagination::DbPagination;
use crate::fetcher::data_request::RedditFeedKind;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::report::report::Report;
use crate::util::get_utc_timestamp;
use axum::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use std::collections::HashSet;

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

    /// Whether include reports belonging to the requesting user.
    #[serde(rename = "mine")]
    pub include_my_reports: Option<bool>,

    /// Filter by the feed kind of the report's data request.
    pub feed_kind: Option<RedditFeedKind>,

    /// Filter by the resource names used in the report's data sources.
    pub resource: Option<String>,

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
            include_my_reports: None,
            feed_kind: None,
            resource: None,
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
#[derive(Debug)]
struct ReportQueryBindArgs {
    /// The user's name pattern.
    ///
    /// If not provided in the query, it's an empty string. It's used as `LIKE %<pattern>%`.
    user_name_pattern: String,
    /// The concatenated `WHERE` clauses for checking if the report contains the specified kinds of analyses.
    ///
    /// If not provided in the query, it's `1=1`, which is always true and maintains the query's correctness.
    contained_analysis_kinds: HashSet<NlpAnalysisKind>,
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
    /// Whether to only fetch reports belonging to the requesting user.
    include_my_reports: bool,
    feed_kind_pattern: String,
    resource_pattern: String,
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
        let user_name_pattern = self.user_name_pattern.unwrap_or("".to_string());
        let contained_analysis_kinds = self.contained_analysis_kinds.into_iter().collect();
        let start_date = self
            .start_date
            .unwrap_or(NaiveDateTime::UNIX_EPOCH.and_utc());
        let end_date = self
            .end_date
            .unwrap_or(get_utc_timestamp() + chrono::Duration::days(1));
        let title_pattern = self.title_pattern.unwrap_or("".to_string());
        let include_my_reports = self.include_my_reports.unwrap_or(false);
        let feed_kind_pattern = self
            .feed_kind
            .map(|f| f.to_snake_case())
            .unwrap_or("".to_string());
        let resource_pattern = self.resource.unwrap_or("".to_string());

        ReportQueryBindArgs {
            user_name_pattern,
            contained_analysis_kinds,
            start_date,
            end_date,
            title_pattern,
            include_my_reports,
            feed_kind_pattern,
            resource_pattern,
        }
    }
}

/// Repository trait for reports.
///
/// Specifies methods for fetching reports from the database.
#[async_trait]
pub trait ReportRepository: Sized {
    async fn get_by_query(
        query: ReportQuery,
        requesting_user_id: GoogleId,
        db: &DbClient,
    ) -> Result<Vec<Self>, DbError>;
}

#[async_trait]
impl ReportRepository for Report {
    /// Fetches reports from the database based on the query and pagination.
    async fn get_by_query(
        query: ReportQuery,
        requesting_user_id: GoogleId,
        db: &DbClient,
    ) -> Result<Vec<Self>, DbError> {
        let (limit, offset) = query.pagination.clone().into_limit_and_offset();
        let bind_args = query.into_bind_args();
        let db_reports = sqlx::query_as!(
            DbReport,
            r#"
            SELECT r.id, user_id, title, description, is_public, is_in_progress, is_successful, is_error, error_message, r.created_at, r.updated_at
            FROM reports r
            JOIN users u ON r.user_id = u.google_id
            JOIN data_requests dr ON r.id = dr.report_id
            WHERE
            position ($1 in u.name) > 0
            AND (r.created_at BETWEEN SYMMETRIC $2 AND $3)
            AND (r.is_public OR r.user_id = $4)
            AND CASE WHEN $5 IS FALSE AND r.user_id = $4 THEN FALSE ELSE TRUE END
            AND position ($6 in r.title) > 0
            AND position ($7 in dr.feed_kind) > 0
            AND EXISTS (
                SELECT 1 
                FROM data_requests dr2
                JOIN data_sources ds ON dr2.id = ds.data_request_id
                WHERE dr2.report_id = r.id
                AND position ($8 in ds.name) > 0 
            )
            AND (
                SELECT CASE WHEN $9 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'clickbait'))
                ELSE TRUE END
                AND (
                    SELECT CASE WHEN $10 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'hate_speech'))
                    ELSE TRUE END
                )
                AND (
                    SELECT CASE WHEN $11 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'keywords'))
                    ELSE TRUE END
                )
                AND (
                    SELECT CASE WHEN $12 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'language'))
                    ELSE TRUE END
                )
                AND (
                    SELECT CASE WHEN $13 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'politics'))
                    ELSE TRUE END
                )
                AND (
                    SELECT CASE WHEN $14 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'sarcasm'))
                    ELSE TRUE END
                )
                AND (
                    SELECT CASE WHEN $15 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'sentiment'))
                    ELSE TRUE END
                )
                AND (
                    SELECT CASE WHEN $16 IS TRUE THEN (SELECT EXISTS (SELECT 1 FROM nlp_analyses WHERE report_id = r.id AND kind = 'spam'))
                    ELSE TRUE END
                )
            )
            LIMIT $17 OFFSET $18
            "#,
            bind_args.user_name_pattern,
            bind_args.start_date,
            bind_args.end_date,
            requesting_user_id,
            bind_args.include_my_reports,
            bind_args.title_pattern,
            bind_args.feed_kind_pattern,
            bind_args.resource_pattern,
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::Clickbait),
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::HateSpeech),
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::Keywords),
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::Language),
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::Politics),
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::Sarcasm),
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::Sentiment),
            bind_args
                .contained_analysis_kinds
                .contains(&NlpAnalysisKind::Spam),
            limit,
            offset
        )
            .fetch_all(db.raw_db())
            .await?;

        let reports_fut = db_reports
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
    use crate::auth::user::User;
    use crate::db::db_stored::DbStored;
    use crate::db::impls::test_util::get_test_user;
    use crate::fetcher::data_request::{DataSource, FetcherDataRequest, RedditFeedKind};
    use crate::fetcher::reddit::request::feed_sorting::{FeedSorting, FeedSortingTime};
    use crate::nlp::nlp_response::NlpAnalysis;
    use crate::report::report_status::ReportStatus;
    use nanoid::nanoid;
    use sqlx::PgPool;
    use std::collections::HashMap;
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
    fn test_into_where_clauses_date_range() {
        let now = get_utc_timestamp();
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
        let now = get_utc_timestamp();
        let query = ReportQuery::default();
        let args = query.into_bind_args();

        assert_eq!(args.start_date, NaiveDateTime::UNIX_EPOCH.and_utc());
        // We need to check the time as well, as the Utc::now() call in the test and the one in the function might be a few milliseconds apart
        // This is why we check if the difference is less than a second
        assert!(args.end_date > now);
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

    fn get_test_reports() -> (Report, Report, Report, Report) {
        let user1 = nanoid!();
        let user2 = nanoid!();
        let user3 = nanoid!();

        let report1 = Report {
            id: nanoid!(),
            user_id: user1.clone(),
            title: "Title 1".to_string(),
            description: "".to_string(),
            is_public: true,
            status: ReportStatus::Success,
            analyses: HashMap::from([(
                NlpAnalysisKind::HateSpeech,
                NlpAnalysis {
                    kind: NlpAnalysisKind::HateSpeech,
                    results: vec![],
                    generated_in: 0.0,
                },
            )]),
            data_request: FetcherDataRequest {
                feed_kind: RedditFeedKind::UserPosts,
                data_sources: vec![
                    DataSource {
                        name: "spez".to_string(),
                        post_id: None,
                        share: 80,
                    },
                    DataSource {
                        name: "spez2".to_string(),
                        post_id: None,
                        share: 20,
                    },
                ],
                size: 10,
                sort_by: FeedSorting::Hot,
            },
            created_at: get_utc_timestamp(),
            updated_at: get_utc_timestamp(),
        };

        let report2 = Report {
            id: nanoid!(),
            user_id: user2,
            title: "Title 2".to_string(),
            description: "Description 2".to_string(),
            is_public: true,
            status: ReportStatus::Success,
            analyses: HashMap::from([
                (
                    NlpAnalysisKind::Sentiment,
                    NlpAnalysis {
                        kind: NlpAnalysisKind::Sentiment,
                        results: vec![],
                        generated_in: 0.0,
                    },
                ),
                (
                    NlpAnalysisKind::Clickbait,
                    NlpAnalysis {
                        kind: NlpAnalysisKind::Clickbait,
                        results: vec![],
                        generated_in: 0.0,
                    },
                ),
            ]),
            data_request: FetcherDataRequest {
                feed_kind: RedditFeedKind::SubredditPosts,
                data_sources: vec![
                    DataSource {
                        name: "Polska".to_string(),
                        post_id: None,
                        share: 50,
                    },
                    DataSource {
                        name: "programming".to_string(),
                        post_id: None,
                        share: 50,
                    },
                ],
                size: 100,
                sort_by: FeedSorting::Controversial(FeedSortingTime::Month),
            },
            created_at: get_utc_timestamp(),
            updated_at: get_utc_timestamp(),
        };

        let report3 = Report {
            id: nanoid!(),
            user_id: user3,
            title: "Title 3".to_string(),
            description: "Description 3".to_string(),
            is_public: true,
            status: ReportStatus::Success,
            analyses: HashMap::from([
                (
                    NlpAnalysisKind::Sentiment,
                    NlpAnalysis {
                        kind: NlpAnalysisKind::Sentiment,
                        results: vec![],
                        generated_in: 0.0,
                    },
                ),
                (
                    NlpAnalysisKind::Politics,
                    NlpAnalysis {
                        kind: NlpAnalysisKind::Politics,
                        results: vec![],
                        generated_in: 0.0,
                    },
                ),
            ]),
            data_request: FetcherDataRequest {
                feed_kind: RedditFeedKind::SubredditPosts,
                data_sources: vec![
                    DataSource {
                        name: "Polska".to_string(),
                        post_id: None,
                        share: 25,
                    },
                    DataSource {
                        name: "programming".to_string(),
                        post_id: None,
                        share: 25,
                    },
                    DataSource {
                        name: "sakratvelo".to_string(),
                        post_id: None,
                        share: 25,
                    },
                    DataSource {
                        name: "europe".to_string(),
                        post_id: None,
                        share: 25,
                    },
                ],
                size: 200,
                sort_by: FeedSorting::Top(FeedSortingTime::All),
            },
            created_at: get_utc_timestamp() - chrono::Duration::days(7),
            updated_at: get_utc_timestamp() - chrono::Duration::days(7),
        };

        let report4 = Report {
            id: nanoid!(),
            user_id: user1,
            title: "Title 4".to_string(),
            description: "Description 4".to_string(),
            is_public: false,
            status: ReportStatus::Success,
            analyses: HashMap::from([(
                NlpAnalysisKind::Sentiment,
                NlpAnalysis {
                    kind: NlpAnalysisKind::Sentiment,
                    results: vec![],
                    generated_in: 0.0,
                },
            )]),
            data_request: FetcherDataRequest {
                feed_kind: RedditFeedKind::SubredditPosts,
                data_sources: vec![DataSource {
                    name: "Polska".to_string(),
                    post_id: Some("1eubxgg".to_string()),
                    share: 100,
                }],
                size: 200,
                sort_by: FeedSorting::Top(FeedSortingTime::All),
            },
            created_at: get_utc_timestamp(),
            updated_at: get_utc_timestamp(),
        };

        (report1, report2, report3, report4)
    }

    async fn setup(db: &DbClient) -> ((Report, Report, Report, Report), (User, User, User)) {
        let (r1, r2, r3, r4) = get_test_reports();
        let user1 = get_test_user(r1.user_id.clone());
        let user2 = get_test_user(r2.user_id.clone());
        let user3 = get_test_user(r3.user_id.clone());

        user1.save(&db).await.unwrap();
        user2.save(&db).await.unwrap();
        user3.save(&db).await.unwrap();

        r1.save(&db).await.unwrap();
        r2.save(&db).await.unwrap();
        r3.save(&db).await.unwrap();
        r4.save(&db).await.unwrap();

        ((r1, r2, r3, r4), (user1, user2, user3))
    }

    #[sqlx::test]
    async fn test_get_by_query_user_name_pattern_all(pool: PgPool) {
        let db = DbClient::new(pool);
        let (_, (u1, _, _)) = setup(&db).await;

        let query = ReportQuery {
            user_name_pattern: Some(u1.name.clone()),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(!reports.is_empty());
        assert!(reports.iter().all(|r| r.user_id.contains(&u1.name)));
    }

    #[sqlx::test]
    async fn test_get_by_query_contained_analysis_kinds_two_sentiments(pool: PgPool) {
        let db = DbClient::new(pool);
        let ((r1, _, _, _), _) = setup(&db).await;

        let query = ReportQuery {
            contained_analysis_kinds: vec![NlpAnalysisKind::Sentiment],
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(reports
            .iter()
            .all(|r| r.analyses.contains_key(&NlpAnalysisKind::Sentiment)));
        assert!(!reports.iter().any(|r| r.id == r1.id));
    }

    #[sqlx::test]
    async fn test_get_by_query_contained_analysis_kinds_one_clickbait(pool: PgPool) {
        let db = DbClient::new(pool);
        let ((_, r2, _, _), _) = setup(&db).await;

        let query = ReportQuery {
            contained_analysis_kinds: vec![NlpAnalysisKind::Clickbait],
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(reports
            .iter()
            .all(|r| r.analyses.contains_key(&NlpAnalysisKind::Clickbait)));
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].id, r2.id);
    }

    #[sqlx::test]
    async fn test_get_by_query_date_range_none_long_ago(pool: PgPool) {
        let db = DbClient::new(pool);
        let _ = setup(&db).await;

        let query = ReportQuery {
            start_date: Some(get_utc_timestamp() - chrono::Duration::days(1000000)),
            end_date: Some(get_utc_timestamp() - chrono::Duration::days(19999)),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert_eq!(reports.len(), 0);
    }

    #[sqlx::test]
    async fn test_get_by_query_date_range_only_r3(pool: PgPool) {
        let db = DbClient::new(pool);
        let ((_, _, r3, _), _) = setup(&db).await;

        let query = ReportQuery {
            start_date: Some(get_utc_timestamp() - chrono::Duration::days(10)),
            end_date: Some(get_utc_timestamp() - chrono::Duration::days(5)),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query.clone(), GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].id, r3.id);
        assert!(reports
            .iter()
            .all(|r| r.created_at >= query.start_date.unwrap()));
        assert!(reports
            .iter()
            .all(|r| r.created_at <= query.end_date.unwrap()));
    }

    #[sqlx::test]
    async fn test_get_by_query_date_range_3_recent(pool: PgPool) {
        let db = DbClient::new(pool);
        let _ = setup(&db).await;

        let query = ReportQuery {
            start_date: Some(get_utc_timestamp() - chrono::Duration::days(1)),
            end_date: Some(get_utc_timestamp() + chrono::Duration::days(1)),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query.clone(), GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert_eq!(reports.len(), 2);
        assert!(reports
            .iter()
            .all(|r| r.created_at >= query.start_date.unwrap()));
        assert!(reports
            .iter()
            .all(|r| r.created_at <= query.end_date.unwrap()));
    }

    #[sqlx::test]
    async fn test_get_by_query_title_pattern_all(pool: PgPool) {
        let db = DbClient::new(pool);
        let _ = setup(&db).await;

        let query = ReportQuery {
            title_pattern: Some(String::from("Title")),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(reports.iter().all(|r| r.title.contains("Title")));
    }

    #[sqlx::test]
    async fn test_get_by_query_title_pattern_one(pool: PgPool) {
        let db = DbClient::new(pool);
        let _ = setup(&db).await;

        let query = ReportQuery {
            title_pattern: Some(String::from("Title 1")),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(reports.iter().all(|r| r.title.contains("Title 1")));
    }

    #[sqlx::test]
    async fn test_get_by_query_include_my_reports_false(pool: PgPool) {
        let db = DbClient::new(pool);
        let ((_, r2, _, _), (_, u2, _)) = setup(&db).await;

        // Catch all except mine
        let query = ReportQuery {
            include_my_reports: Some(false),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, u2.clone().id, &db)
            .await
            .unwrap();

        assert_eq!(reports.len(), 2); // 2 because the 4th one is private, and 2nd is ours
        assert!(!reports.contains(&r2));
    }

    #[sqlx::test]
    async fn test_get_by_query_private_unauthorized(pool: PgPool) {
        let db = DbClient::new(pool);
        let ((_, _, _, r4), _) = setup(&db).await;

        let query = ReportQuery::default();

        let reports = Report::get_by_query(query, GoogleId::from("Random User 123"), &db)
            .await
            .unwrap();

        assert_eq!(reports.len(), 3);
        assert!(reports.iter().all(|r| r.is_public));
        assert!(!reports.contains(&r4));
    }

    #[sqlx::test]
    /// Report 4 is private. User 1 is the owner, so they should be able to see it.
    async fn test_get_by_query_private_authorized(pool: PgPool) {
        let db = DbClient::new(pool);
        let ((_, _, _, r4), (u1, _, _)) = setup(&db).await;

        let query = ReportQuery {
            include_my_reports: Some(true),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, u1.id, &db).await.unwrap();

        assert!(reports.contains(&r4));
        assert_eq!(reports.len(), 4);
    }

    #[sqlx::test]
    async fn test_get_by_query_feed_kind_subreddit_posts(pool: PgPool) {
        let db = DbClient::new(pool);
        let _ = setup(&db).await;

        let query = ReportQuery {
            feed_kind: Some(RedditFeedKind::SubredditPosts),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(reports
            .iter()
            .all(|r| r.data_request.feed_kind == RedditFeedKind::SubredditPosts));
        assert_eq!(reports.len(), 2);
    }

    #[sqlx::test]
    async fn test_get_by_query_feed_kind_user_posts(pool: PgPool) {
        let db = DbClient::new(pool);
        let _ = setup(&db).await;

        let query = ReportQuery {
            feed_kind: Some(RedditFeedKind::UserPosts),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(reports
            .iter()
            .all(|r| r.data_request.feed_kind == RedditFeedKind::UserPosts));
        assert_eq!(reports.len(), 1);
    }

    #[sqlx::test]
    async fn test_get_by_query_resource(pool: PgPool) {
        let db = DbClient::new(pool);
        let _ = setup(&db).await;

        let query = ReportQuery {
            resource: Some("Polska".to_string()),
            ..ReportQuery::default()
        };

        let reports = Report::get_by_query(query, GoogleId::from("test_user"), &db)
            .await
            .unwrap();

        assert!(reports.iter().all(|r| r
            .data_request
            .data_sources
            .iter()
            .any(|ds| ds.name == "Polska")));
        assert_eq!(reports.len(), 2);
    }
}
