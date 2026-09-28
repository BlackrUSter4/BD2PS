use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameBingoInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "ClearCount")]
    pub clear_count: Option<i32>,
    #[sqlx(rename = "BingoBoard")]
    pub bingo_board: String,
    #[sqlx(rename = "OpenBingoBoardIndex")]
    pub open_bingo_board_index: String,
}
