use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct DefenseUnitInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UnitId")]
    pub unit_id: Option<i32>,
    #[sqlx(rename = "UnitIndex")]
    pub unit_index: Option<i32>,
    #[sqlx(rename = "SpawnTicks")]
    pub spawn_ticks: Option<i64>,
    #[sqlx(rename = "DespwanTicks")]
    pub despwan_ticks: Option<i64>,
    #[sqlx(rename = "ElementLevel")]
    pub element_level: Option<i32>,
    #[sqlx(rename = "CoolTime")]
    pub cool_time: Option<f32>,
    #[sqlx(rename = "GridIndex")]
    pub grid_index: Option<i32>,
}