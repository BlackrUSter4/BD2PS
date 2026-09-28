use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PresetDeckEquipInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "PresetDeckInfoIndex")]
    pub preset_deck_info_index: i64,
    #[sqlx(rename = "EquipType")]
    pub equip_type: Option<i32>,
    #[sqlx(rename = "EquipInvenIndex")]
    pub equip_inven_index: Option<i64>,
}
