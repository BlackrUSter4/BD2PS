use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct BattleGridTypeItemInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "TeamType")]
    pub team_type: Option<i32>,
    #[sqlx(rename = "GridIndex")]
    pub grid_index: Option<i32>,
    #[sqlx(rename = "BuffId")]
    pub buff_id: Option<i32>,
    #[sqlx(rename = "IsInfinite")]
    pub is_infinite: Option<bool>,
    #[sqlx(rename = "BuffTurn")]
    pub buff_turn: Option<i32>,
}