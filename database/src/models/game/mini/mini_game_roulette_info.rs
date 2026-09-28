use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameRouletteInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "FreeApCount")]
    pub free_ap_count: Option<i32>,
    #[sqlx(rename = "ResetTime")]
    pub reset_time: Option<i64>,
    #[sqlx(rename = "IsRewardSpecialItem")]
    pub is_reward_special_item: Option<bool>,
    #[sqlx(rename = "TryCount")]
    pub try_count: Option<i32>,
}