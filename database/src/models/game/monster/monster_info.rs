use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "MonsterId")]
    pub monster_id: Option<i32>,
    #[sqlx(rename = "BattleDeck")]
    pub battle_deck: Option<i32>,
    #[sqlx(rename = "RespawnTime")]
    pub respawn_time: Option<i64>,
    #[sqlx(rename = "LifeEndTime")]
    pub life_end_time: Option<i64>,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "ActiveFlag")]
    pub active_flag: Option<bool>,
}