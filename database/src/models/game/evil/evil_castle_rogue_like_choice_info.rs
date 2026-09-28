use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleRogueLikeChoiceInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Type")]
    pub r#type: Option<i32>,
    /// JSON-encoded Vec<i32> — the proto field is `repeated int32 id`, but
    /// the original migration only had a `Type` column (latent schema gap).
    #[sqlx(rename = "Ids")]
    pub ids: Option<String>,
}