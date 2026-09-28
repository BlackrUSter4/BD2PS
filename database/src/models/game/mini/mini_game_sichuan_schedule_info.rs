use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameSichuanScheduleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "InfoIndex")]
    pub info_index: Option<String>,
    #[sqlx(rename = "WorldBestRecordOwnerIndex")]
    pub world_best_record_owner_index: Option<i64>,
    #[sqlx(rename = "WorldBestRecordUserId")]
    pub world_best_record_user_id: Option<String>,
    #[sqlx(rename = "WorldBestRecordValue")]
    pub world_best_record_value: Option<f64>,
    #[sqlx(rename = "BestRecordValue")]
    pub best_record_value: Option<f64>,
    #[sqlx(rename = "RewardInfoIndex")]
    pub reward_info_index: Option<String>,
    #[sqlx(rename = "ActiveInfoIndex")]
    pub active_info_index: Option<String>,
    #[sqlx(rename = "IsBlock")]
    pub is_block: Option<i32>,
}