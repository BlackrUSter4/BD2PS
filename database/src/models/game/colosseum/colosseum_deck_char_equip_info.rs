use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ColosseumDeckCharEquipInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CharInvenIndex")]
    pub char_inven_index: i64,
    #[sqlx(rename = "EquipType")]
    pub equip_type: Option<i32>,
    #[sqlx(rename = "EquipInvenIndex")]
    pub equip_inven_index: Option<i64>,
}
