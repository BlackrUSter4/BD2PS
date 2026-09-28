use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct OverwhelmMonsterInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "MonsterId")]
    pub monster_id: Option<i32>,
    #[sqlx(rename = "BattleDeck")]
    pub battle_deck: Option<i32>,
    #[sqlx(rename = "BattleMode")]
    pub battle_mode: Option<i32>,
}