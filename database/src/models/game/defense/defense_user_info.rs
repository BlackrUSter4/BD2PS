use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct DefenseUserInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "State")]
    pub state: Option<i32>,
    #[sqlx(rename = "EnemyCount")]
    pub enemy_count: Option<i32>,
    #[sqlx(rename = "Wave")]
    pub wave: Option<i32>,
    #[sqlx(rename = "TotalEnemyKillCount")]
    pub total_enemy_kill_count: Option<i32>,
    #[sqlx(rename = "NetworkState")]
    pub network_state: Option<i32>,
    #[sqlx(rename = "NetworkPing")]
    pub network_ping: Option<i32>,
}