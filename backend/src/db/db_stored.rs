use crate::db::db_client::DbClient;
use crate::db::pagination::DbPagination;
use axum::async_trait;
use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;

async fn handle_db_result<T>(
    result: Result<T, Error>,
    tx: Transaction<'_, Postgres>,
) -> Result<T, Error> {
    match result {
        Ok(result) => {
            log::info!("Committing transaction");
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            log::error!("Transaction failed: {:?}", e);
            let r = tx.rollback().await;
            if let Err(e) = r {
                log::error!("Rollback failed: {:?}", e);
            } else {
                log::warn!("Transaction rolled back");
            }
            Err(e)
        }
    }
}

#[allow(private_bounds)]
pub trait DbStored: DbStoredInner {
    async fn save(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_save(&mut tx).await;
        handle_db_result(result, tx).await
    }
    async fn update(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_update(&mut tx).await;
        handle_db_result(result, tx).await
    }
    async fn delete(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_delete(&mut tx).await;
        handle_db_result(result, tx).await
    }
    async fn get_by_id(id: &str, db: &DbClient) -> Result<Option<Self>, Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = Self::inner_get_by_id(id, &mut tx).await?;
        handle_db_result(Ok(result), tx).await
    }
    async fn get_all(pagination: DbPagination, db: &DbClient) -> Result<Vec<Self>, Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = Self::inner_get_all(pagination, &mut tx).await?;
        handle_db_result(Ok(result), tx).await
    }
}

impl<T> DbStored for T where T: DbStoredInner {}

#[async_trait]
pub(super) trait DbStoredInner: Sized {
    async fn inner_save(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;
    async fn inner_update(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;
    async fn inner_delete(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;

    async fn inner_get_by_id(
        id: &str,
        db: &mut Transaction<Postgres>,
    ) -> Result<Option<Self>, Error>;

    async fn inner_get_all(
        pagination: DbPagination,
        db: &mut Transaction<Postgres>,
    ) -> Result<Vec<Self>, Error>;
}

#[async_trait]
pub(super) trait DbStoredDependentlyInner: Sized {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error>;
}
