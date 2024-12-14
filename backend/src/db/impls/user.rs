use crate::auth::google::User;
use crate::db::db_stored::DbStoredInner;
use crate::db::pagination::DbPagination;
use axum::async_trait;
use sqlx::{Error, PgPool, Postgres, Transaction};

#[async_trait]
impl DbStoredInner for User {
    async fn inner_save(&self, tx: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (
            google_sub, name, given_name, family_name, picture, email, email_verified
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (google_sub) DO UPDATE
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
    async fn inner_update(&self, tx: &mut Transaction<Postgres>) -> Result<(), Error> {
        unimplemented!()
    }
    async fn inner_delete(&self, tx: &mut Transaction<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM users WHERE google_sub = $1
            "#,
            self.id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn inner_get_by_id(id: &str, pool: &PgPool) -> Result<Option<Self>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT google_sub as "id: String", name, given_name, family_name, picture, email, email_verified
            FROM users
            WHERE google_sub = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    async fn inner_get_all(pagination: DbPagination, pool: &PgPool) -> Result<Vec<Self>, Error> {
        unimplemented!()
    }
}
