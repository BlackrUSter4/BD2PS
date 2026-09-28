use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "AttackDeckInfoIndex")]
    pub attack_deck_info_index: Option<String>,
    #[sqlx(rename = "AttackDeckItemInfoIndex")]
    pub attack_deck_item_info_index: Option<String>,
    #[sqlx(rename = "DefenseDeckInfoIndex")]
    pub defense_deck_info_index: Option<String>,
    #[sqlx(rename = "DefenseDeckItemInfoIndex")]
    pub defense_deck_item_info_index: Option<String>,
}