use serde::{Deserialize, Serialize};

pub type GoogleId = String;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct User {
    /// Unique user ID
    #[serde(rename = "sub")]
    #[sqlx(rename = "google_id")]
    pub id: GoogleId,
    pub name: String,
    pub given_name: String,
    pub family_name: Option<String>,
    /// URL to the user's picture
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
}
