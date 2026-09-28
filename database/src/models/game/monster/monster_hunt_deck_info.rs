use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterHuntDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Team")]
    pub team: Option<i32>,
    #[sqlx(rename = "DeckInfoIndex")]
    pub deck_info_index: Option<String>,
    #[sqlx(rename = "BattlePower")]
    pub battle_power: Option<i32>,
}
