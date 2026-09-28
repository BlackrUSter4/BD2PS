use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct BattleStatisticsInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "AttackDamageInfoIndex")]
    pub attack_damage_info_index: Option<String>,
    #[sqlx(rename = "ReceivedDamageInfoIndex")]
    pub received_damage_info_index: Option<String>,
}
