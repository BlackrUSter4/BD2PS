use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct NpcQuizClearInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventUid")]
    pub event_uid: i32,
    #[sqlx(rename = "GroupId")]
    pub group_id: i32,
    #[sqlx(rename = "Id")]
    pub id: i32,
}
