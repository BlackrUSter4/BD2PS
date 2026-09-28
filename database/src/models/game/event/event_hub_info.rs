use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EventHubInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: Option<i64>,
    #[sqlx(rename = "HubId")]
    pub hub_id: Option<i32>,
    #[sqlx(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[sqlx(rename = "PlayEndTime")]
    pub play_end_time: Option<i64>,
    #[sqlx(rename = "EndTime")]
    pub end_time: Option<i64>,
    #[sqlx(rename = "SettingInfoIndex")]
    pub setting_info_index: Option<String>,
}
