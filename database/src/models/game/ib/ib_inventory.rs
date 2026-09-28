use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct IbInventory {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: i64,
    #[sqlx(rename = "Type")]
    pub r#type: i32,
    #[sqlx(rename = "ItemId")]
    pub item_id: i32,
    #[sqlx(rename = "Level")]
    pub level: i32,
}
