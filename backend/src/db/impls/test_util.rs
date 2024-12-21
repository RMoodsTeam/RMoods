use crate::auth::user::GoogleId;
use crate::db::db_error::DbError;
use sqlx::PgPool;

pub(super) async fn insert_test_user(pool: &PgPool) -> Result<GoogleId, DbError> {
    let id = sqlx::query!(
            r#"
            INSERT INTO users (
            google_sub, name, given_name, family_name, picture, email, email_verified
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (google_sub) DO UPDATE
            SET name = $2, given_name = $3, family_name = $4, picture = $5, email = $6, email_verified = $7
            RETURNING google_sub as "id: String";
            "#,
            "test_user_id",
            "Test User",
            "Test",
            "User",
            "https://example.com/picture",
            "test@example.com",
            true
        )
        .fetch_one(pool)
        .await?.id;

    Ok(id)
}
