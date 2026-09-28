use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildActionInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Type")]
    pub r#type: Option<serde_json::Value>,
    #[sqlx(rename = "Time")]
    pub time: Option<i64>,
    #[sqlx(rename = "GuildId")]
    pub guild_id: Option<i64>,
    #[sqlx(rename = "GuildName")]
    pub guild_name: Option<String>,
    #[sqlx(rename = "IsNotify")]
    pub is_notify: Option<i32>,
    #[sqlx(rename = "Role")]
    pub role: Option<serde_json::Value>,
}