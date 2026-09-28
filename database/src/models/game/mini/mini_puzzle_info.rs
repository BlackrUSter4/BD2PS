use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniPuzzleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "ClearCount")]
    pub clear_count: Option<i32>,
    #[sqlx(rename = "Puzzle")]
    pub puzzle: i32,
    #[sqlx(rename = "PuzzleOpen")]
    pub puzzle_open: i32,
}