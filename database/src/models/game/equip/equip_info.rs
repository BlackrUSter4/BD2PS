use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EquipInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "UseChar")]
    pub use_char: Option<i64>,
    #[sqlx(rename = "KeepFlag")]
    pub keep_flag: Option<i32>,
    #[sqlx(rename = "LockFlag")]
    pub lock_flag: Option<i32>,
    #[sqlx(rename = "BaseInfoIndex")]
    pub base_info_index: Option<i64>,
    #[sqlx(rename = "Mark")]
    pub mark: Option<String>,
}