use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct BattleGolemInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "Gauge")]
    pub gauge: Option<f64>,
    #[sqlx(rename = "RemainTurn")]
    pub remain_turn: Option<i32>,
    #[sqlx(rename = "ReserveCostumeId")]
    pub reserve_costume_id: Option<i32>,
    #[sqlx(rename = "Key")]
    pub key: i32,
    #[sqlx(rename = "Value")]
    pub value: i32,
}