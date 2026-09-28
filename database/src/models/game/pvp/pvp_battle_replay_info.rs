use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleReplayInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BlueDeckFullInfoIndex")]
    pub blue_deck_full_info_index: Option<i64>,
    #[sqlx(rename = "RedDeckFullInfoIndex")]
    pub red_deck_full_info_index: Option<i64>,
    #[sqlx(rename = "BattleRandomSeed")]
    pub battle_random_seed: i32,
}