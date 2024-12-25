use crate::auth::user::User;
use crate::db::db_error::DbError;
use crate::db::db_stored::DbStoredInner;
use crate::db::pagination::DbPagination;
use axum::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

#[async_trait]
impl DbStoredInner for User {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            INSERT INTO users (
            google_id, name, given_name, family_name, picture, email, email_verified
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (google_id) DO UPDATE
            SET name = $2, given_name = $3, family_name = $4, picture = $5, email = $6, email_verified = $7
            "#,
            self.id,
            self.name,
            self.given_name,
            self.family_name,
            self.picture,
            self.email,
            self.email_verified
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
    async fn inner_update(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
        unimplemented!()
    }
    async fn inner_delete(&self, tx: &mut Transaction<Postgres>) -> Result<(), DbError> {
        // Delete all reports of that user
        sqlx::query!(
            r#"
            DELETE FROM reports WHERE user_id = $1
            "#,
            self.id
        )
        .execute(&mut **tx)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM users WHERE google_id = $1
            "#,
            self.id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn inner_get_by_id(id: &str, pool: &PgPool) -> Result<Option<Self>, DbError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT google_id as "id: String", name, given_name, family_name, picture, email, email_verified
            FROM users
            WHERE google_id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    async fn inner_get_all(pagination: DbPagination, pool: &PgPool) -> Result<Vec<Self>, DbError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::db_stored::DbStored;
    use crate::db::impls::test_util::{get_db, get_test_user};
    use serial_test::serial;

    #[sqlx::test]
    #[serial]
    async fn test_user_save() {
        let db = get_db().await;
        let user = get_test_user();

        user.save(&db).await.unwrap();
        let user_from_db = User::get_by_id(&user.id, &db).await.unwrap().unwrap();
        assert_eq!(user, user_from_db);

        user.delete(&db).await.unwrap();
        let user_from_db = User::get_by_id(&user.id, &db).await.unwrap();
        assert!(user_from_db.is_none());
    }
}
