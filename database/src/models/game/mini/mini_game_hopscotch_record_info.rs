use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameHopscotchRecord {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: i32,
    #[sqlx(rename = "StageId")]
    pub stage_id: i32,
    #[sqlx(rename = "IsClear")]
    pub is_clear: bool,
    #[sqlx(rename = "CapturedArea")]
    pub captured_area: i32,
    #[sqlx(rename = "ClearTime")]
    pub clear_time: i32,
}
