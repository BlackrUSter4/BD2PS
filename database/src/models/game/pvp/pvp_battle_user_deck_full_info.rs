use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleUserDeckFullInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "DeckInfoIndex")]
    pub deck_info_index: Option<String>,
    #[sqlx(rename = "CharInfoIndex")]
    pub char_info_index: Option<String>,
    #[sqlx(rename = "CostumeInfoIndex")]
    pub costume_info_index: Option<String>,
    #[sqlx(rename = "EquipInfoIndex")]
    pub equip_info_index: Option<String>,
    #[sqlx(rename = "BuffStatInfoIndex")]
    pub buff_stat_info_index: Option<String>,
    #[sqlx(rename = "AwakeInfoIndex")]
    pub awake_info_index: Option<String>,
}