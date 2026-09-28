use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct DatingEpisodeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "DatingPoint")]
    pub dating_point: Option<i32>,
    #[sqlx(rename = "LastClearId")]
    pub last_clear_id: Option<i32>,
    #[sqlx(rename = "LastMessageGroupId")]
    pub last_message_group_id: Option<i32>,
    #[sqlx(rename = "LastMessageId")]
    pub last_message_id: Option<i32>,
    #[sqlx(rename = "LastMessageUpdateTime")]
    pub last_message_update_time: Option<i64>,
}