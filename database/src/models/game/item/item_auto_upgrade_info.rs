use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ItemAutoUpgradeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "ItemType")]
    pub item_type: Option<i32>,
    #[sqlx(rename = "ItemId")]
    pub item_id: Option<i32>,
    #[sqlx(rename = "BeforeLevel")]
    pub before_level: Option<i32>,
    #[sqlx(rename = "AfterLevel")]
    pub after_level: Option<i32>,
    #[sqlx(rename = "SortId")]
    pub sort_id: Option<i32>,
}