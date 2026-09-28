use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EventHubSettingInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "HubInfoIndex")]
    pub hub_info_index: i64,
    #[sqlx(rename = "Slot")]
    pub slot: Option<i32>,
    #[sqlx(rename = "HubContentType")]
    pub hub_content_type: Option<i32>,
    #[sqlx(rename = "EventUid")]
    pub event_uid: i32,
}