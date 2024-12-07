use crate::db::db_client::DbClient;
use crate::db::pagination::DbPagination;
use axum::async_trait;
use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;

/// Handles the result of a database operation, committing the transaction if successful and
/// rolling back if not.
///
/// Also logs the result of the transaction.
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

/// The public-facing trait for database-stored objects.
///
/// It's implemented only for the top-level types, such as the [Report](crate::nlp::report::Report)
/// or [User](crate::auth::user::User) structs.
///
/// The actual implementation is done in the [DbStoredInner](crate::db::db_stored::DbStoredInner) trait.
/// DbStored just provides the public API and handles transactions and on-fail rollbacks.
///
/// Two methods for fetching data are provided, but more specific ones should be added separately,
/// maybe in new type-specific traits.
#[allow(private_bounds)]
pub trait DbStored: DbStoredInner {
    /// Saves the object to the database.
    /// Rollbacks the transaction should the operation fail.
    async fn save(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_save(&mut tx).await;
        handle_db_result(result, tx).await
    }
    /// Updates the object in the database.
    /// Rollbacks the transaction should the operation fail.
    async fn update(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_update(&mut tx).await;
        handle_db_result(result, tx).await
    }
    /// Deletes the object from the database.
    /// Rollbacks the transaction should the operation fail.
    async fn delete(&self, db: &DbClient) -> Result<(), Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = self.inner_delete(&mut tx).await;
        handle_db_result(result, tx).await
    }
    /// Fetches the object by its ID.
    /// Rollbacks the transaction should the operation fail.
    async fn get_by_id(id: &str, db: &DbClient) -> Result<Option<Self>, Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = Self::inner_get_by_id(id, &mut tx).await?;
        handle_db_result(Ok(result), tx).await
    }
    /// Fetches all objects of this type.
    /// Rollbacks the transaction should the operation fail.
    async fn get_all(pagination: DbPagination, db: &DbClient) -> Result<Vec<Self>, Error> {
        let mut tx = db.raw_db().begin().await?;
        let result = Self::inner_get_all(pagination, &mut tx).await?;
        handle_db_result(Ok(result), tx).await
    }
}
impl<T> DbStored for T where T: DbStoredInner {}

/// Internal trait for database-stored objects.
///
/// As opposed to the [DbStored](crate::db::db_stored::DbStored) trait, this one has concrete implementations
/// for top-level types - [Report](crate::nlp::report::Report) and [User](crate::auth::user::User).
#[async_trait]
pub(super) trait DbStoredInner: Sized {
    /// Saves the object to the database using the provided transaction.
    async fn inner_save(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;
    /// Updates the object in the database using the provided transaction.
    async fn inner_update(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;
    /// Deletes the object from the database using the provided transaction.
    async fn inner_delete(&self, db: &mut Transaction<Postgres>) -> Result<(), Error>;

    /// Fetches the object by its ID using the provided transaction.
    async fn inner_get_by_id(
        id: &str,
        db: &mut Transaction<Postgres>,
    ) -> Result<Option<Self>, Error>;

    /// Fetches all objects of this type using the provided transaction.
    async fn inner_get_all(
        pagination: DbPagination,
        db: &mut Transaction<Postgres>,
    ) -> Result<Vec<Self>, Error>;
}

/// Internal trait for database-stored objects that depend on other objects.
///
/// This is used for objects that are referenced by other objects, such as with [Report](crate::nlp::report::Report) and its [ReportMetadata](crate::nlp::report::ReportMetadata).
/// The latter is saved first (using this trait) - and its UUID is then used to save the former (using the [DbStored](crate::db::db_stored::DbStored) trait).
#[async_trait]
pub(super) trait DbStoredDependentlyInner: Sized {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<Uuid, Error>;
}
