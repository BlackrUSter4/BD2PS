use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleUserBaseInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Vp")]
    pub vp: Option<i32>,
    #[sqlx(rename = "Rank")]
    pub rank: Option<i32>,
    #[sqlx(rename = "WinCount")]
    pub win_count: Option<i32>,
    #[sqlx(rename = "LoseCount")]
    pub lose_count: Option<i32>,
}