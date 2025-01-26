use crate::db::db_error::DbError;
use crate::db::db_stored::DbStoredDependentlyInner;
use crate::db::from_db::FromDb;
use crate::db::model::{DbDataRequest, DbDataSource};
use crate::fetcher::data_request::{DataSource, FetcherDataRequest, RedditFeedKind};
use crate::fetcher::reddit::request::feed_sorting::{FeedSorting, FeedSortingTime};
use crate::validation::validation_error::ValidationError;
use futures_util::future::join_all;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

fn feed_sorting_into_snake_case(feed_sorting: FeedSorting) -> (String, Option<String>) {
    match feed_sorting {
        FeedSorting::Hot => ("hot".to_string(), None),
        FeedSorting::New => ("new".to_string(), None),
        FeedSorting::Rising => ("rising".to_string(), None),
        FeedSorting::Top(time) => ("top".to_string(), Some(time.to_string())),
        FeedSorting::Controversial(time) => ("controversial".to_string(), Some(time.to_string())),
    }
}

fn feed_sorting_from_snake_case(kind: &str, time: Option<&str>) -> Result<FeedSorting, DbError> {
    let time_from_str = time.map(|t| match t {
        "hour" => Ok(FeedSortingTime::Hour),
        "day" => Ok(FeedSortingTime::Day),
        "week" => Ok(FeedSortingTime::Week),
        "month" => Ok(FeedSortingTime::Month),
        "year" => Ok(FeedSortingTime::Year),
        "all" => Ok(FeedSortingTime::All),
        _ => Err(ValidationError::Invalid("time".to_string())),
    });

    match kind {
        "hot" => Ok(FeedSorting::Hot),
        "new" => Ok(FeedSorting::New),
        "rising" => Ok(FeedSorting::Rising),
        "top" => {
            if let Ok(time) = time_from_str.unwrap() {
                return Ok(FeedSorting::Top(time));
            }
            Err(DbError::from(ValidationError::Invalid(
                "Invalid time value in DB".to_string(),
            )))
        }
        "controversial" => {
            if let Ok(time) = time_from_str.unwrap() {
                return Ok(FeedSorting::Controversial(time));
            }
            Err(DbError::from(ValidationError::Invalid(
                "Invalid time value in DB".to_string(),
            )))
        }
        _ => Err(DbError::from(ValidationError::Invalid(
            "Invalid kind in DB".to_string(),
        ))),
    }
}

impl DbStoredDependentlyInner for FetcherDataRequest {
    async fn inner_save(
        &self,
        parent_id: &str,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Uuid, DbError> {
        let data_request_uuid = sqlx::query!(
            r#"
            INSERT INTO data_requests (report_id, feed_kind, size, sort_by_kind, sort_by_time)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id as "id: Uuid"
            "#,
            parent_id,
            self.feed_kind.to_snake_case(),
            self.size as i32,
            feed_sorting_into_snake_case(self.sort_by).0,
            feed_sorting_into_snake_case(self.sort_by).1
        )
        .fetch_one(&mut **tx)
        .await?
        .id;

        for source in &self.data_sources {
            source
                .inner_save(data_request_uuid.to_string().as_str(), tx)
                .await?;
        }

        Ok(data_request_uuid)
    }
}

impl FromDb for FetcherDataRequest {
    type DbModel = DbDataRequest;
    async fn from_db_model(model: Self::DbModel, pool: &PgPool) -> Result<Self, DbError> {
        let db_data_sources = sqlx::query_as!(
            DbDataSource,
            r#"
            SELECT *
            FROM data_sources
            WHERE data_request_id = $1
            "#,
            model.id,
        )
        .fetch_all(pool)
        .await?;

        let data_sources = join_all(
            db_data_sources
                .into_iter()
                .map(|source| async { DataSource::from_db_model(source, pool).await })
                .collect::<Vec<_>>(),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        Ok(FetcherDataRequest {
            feed_kind: RedditFeedKind::from_snake_case(&model.feed_kind)?,
            size: model.size as u16,
            sort_by: feed_sorting_from_snake_case(
                &model.sort_by_kind,
                model.sort_by_time.as_deref(),
            )?,
            data_sources,
        })
    }
}
