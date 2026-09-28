use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "LastRewardPoint")]
    pub last_reward_point: Option<i32>,
    #[sqlx(rename = "BestRecordValue")]
    pub best_record_value: Option<i32>,
    #[sqlx(rename = "IsPossibleQuickReward")]
    pub is_possible_quick_reward: Option<bool>,
    #[sqlx(rename = "WorldBestRecordOwnerIndex")]
    pub world_best_record_owner_index: Option<i64>,
    #[sqlx(rename = "WorldBestRecordUserId")]
    pub world_best_record_user_id: Option<String>,
    #[sqlx(rename = "WorldBestRecordValue")]
    pub world_best_record_value: Option<i32>,
    #[sqlx(rename = "WorldBestRecordPlayInfo")]
    pub world_best_record_play_info: Option<String>,
}