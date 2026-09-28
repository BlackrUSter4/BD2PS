use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct DefenseMonsterInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "MonsterId")]
    pub monster_id: Option<i32>,
    #[sqlx(rename = "MonsterIndex")]
    pub monster_index: Option<i32>,
    #[sqlx(rename = "SpawnTicks")]
    pub spawn_ticks: Option<i64>,
    #[sqlx(rename = "DespwanTicks")]
    pub despwan_ticks: Option<i64>,
    #[sqlx(rename = "Health")]
    pub health: Option<i32>,
}