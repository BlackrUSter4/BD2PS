use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleUserDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CharInvenIndex")]
    pub char_inven_index: Option<i64>,
    #[sqlx(rename = "Position")]
    pub position: Option<i32>,
    #[sqlx(rename = "Sequence")]
    pub sequence: Option<i32>,
    #[sqlx(rename = "CostumeInvenIndex")]
    pub costume_inven_index: Option<i64>,
    #[sqlx(rename = "CostumeInvenIndexSeq")]
    pub costume_inven_index_seq: i64,
    #[sqlx(rename = "PrioritySkillCostumeInvenIndex")]
    pub priority_skill_costume_inven_index: i64,
}