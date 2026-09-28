use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct SupporterInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "SupporterInfoIndex")]
    pub supporter_info_index: Option<String>,
    #[sqlx(rename = "GuildSupporterInfoIndex")]
    pub guild_supporter_info_index: Option<String>,
}