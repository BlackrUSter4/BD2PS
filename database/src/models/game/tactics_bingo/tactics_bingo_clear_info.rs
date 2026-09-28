use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct TacticsBingoClearInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: i32,
    #[sqlx(rename = "GroupId")]
    pub group_id: i32,
    /// Comma-separated cleared stage ids.
    #[sqlx(rename = "ClearStage")]
    pub clear_stage: Option<String>,
    #[sqlx(rename = "EventFlag")]
    pub event_flag: Option<i32>,
}
