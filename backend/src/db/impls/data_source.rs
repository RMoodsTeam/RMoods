use crate::db::db_error::DbError;
use crate::db::db_stored::DbStoredDependentlyInner;
use crate::db::from_db::FromDb;
use crate::db::model::DbDataSource;
use crate::fetcher::data_request::DataSource;
use crate::validation::validation_error::ValidationError;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

impl DbStoredDependentlyInner for DataSource {
    async fn inner_save(
        &self,
        parent_id: &str,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Uuid, DbError> {
        let parent_id = Uuid::parse_str(parent_id).map_err(|_| {
            DbError::from(ValidationError::Invalid(
                "parent_id not a valid UUID".to_string(),
            ))
        })?;

        let id = sqlx::query!(
            r#"
            INSERT INTO data_sources (data_request_id, name, post_id, share)
            VALUES ($1, $2, $3, $4)
            RETURNING id as "id: Uuid"
            "#,
            parent_id,
            self.name,
            self.post_id,
            self.share as i32
        )
        .fetch_one(&mut **tx)
        .await?
        .id;

        Ok(id)
    }
}

impl FromDb for DataSource {
    type DbModel = DbDataSource;
    async fn from_db_model(model: Self::DbModel, _pool: &PgPool) -> Result<Self, DbError> {
        Ok(Self {
            name: model.name,
            post_id: model.post_id,
            share: model.share as u8,
        })
    }
}
