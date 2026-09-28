use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ContentsCharItemInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CharInvenIndex")]
    pub char_inven_index: Option<i64>,
    #[sqlx(rename = "EquipInfoIndex")]
    pub equip_info_index: Option<String>,
    #[sqlx(rename = "ConnectPotentialCostume")]
    pub connect_potential_costume: Option<i32>,
}
