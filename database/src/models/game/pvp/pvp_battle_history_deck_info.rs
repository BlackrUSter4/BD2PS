use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleHistoryDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UserDeckFullInfoIndex")]
    pub user_deck_full_info_index: Option<i64>,
    #[sqlx(rename = "EnemyDeckFullInfoIndex")]
    pub enemy_deck_full_info_index: Option<i64>,
}