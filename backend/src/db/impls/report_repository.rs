use crate::auth::google::GoogleId;
use crate::db::db_client::DbClient;
use crate::db::from_db::FromDb;
use crate::db::model::DbReport;
use crate::db::pagination::DbPagination;
use crate::nlp::analysis::NlpAnalysisKind;
use crate::nlp::report::Report;
use axum::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Error;

pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

pub struct ReportQuery {
    pub user_id: Option<GoogleId>,
    pub contained_analysis_kinds: Option<Vec<NlpAnalysisKind>>,
    pub date_range: Option<DateRange>,
    pub title_pattern: Option<String>,
}

pub(in crate::db) struct ReportQueryWhereClauses {
    user_id: String,
    contained_analysis_kinds: String,
    date_range: String,
    title_pattern: String,
}

impl ReportQuery {
    pub(in crate::db) fn into_where_clauses(self) -> ReportQueryWhereClauses {
        let noop = "1=1".to_string();

        let user_id_clause = self
            .user_id
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

        let where_clauses = query.into_where_clauses();

        let query: Vec<DbReport> = sqlx::query_as(
            r#"
        SELECT * FROM reports
        WHERE ? AND ? AND ? AND ?
        LIMIT ? OFFSET ?
        "#,
        )
        .bind(where_clauses.user_id)
        .bind(where_clauses.contained_analysis_kinds)
        .bind(where_clauses.date_range)
        .bind(where_clauses.title_pattern)
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
