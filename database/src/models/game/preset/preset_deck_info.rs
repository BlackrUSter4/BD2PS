use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PresetDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "PresetInfoIndex")]
    pub preset_info_index: i64,
    #[sqlx(rename = "CharInvenIndex")]
    pub char_inven_index: i64,
    #[sqlx(rename = "Position")]
    pub position: Option<i32>,
    #[sqlx(rename = "Sequence")]
    pub sequence: Option<i32>,
    #[sqlx(rename = "CostumeInvenIndex")]
    pub costume_inven_index: Option<i64>,
    #[sqlx(rename = "Team")]
    pub team: Option<i32>,
}
