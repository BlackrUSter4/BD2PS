use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GuildBaseInfoIndex")]
    pub guild_base_info_index: Option<i64>,
    #[sqlx(rename = "JoinType")]
    pub join_type: Option<serde_json::Value>,
    #[sqlx(rename = "Message")]
    pub message: Option<String>,
    #[sqlx(rename = "UpdateDate")]
    pub update_date: Option<i64>,
    #[sqlx(rename = "Date")]
    pub date: Option<i64>,
    #[sqlx(rename = "MemberCount")]
    pub member_count: Option<i32>,
    #[sqlx(rename = "DeleteRemainingTime")]
    pub delete_remaining_time: Option<i64>,
    #[sqlx(rename = "NoticeUpdateDate")]
    pub notice_update_date: Option<i64>,
}