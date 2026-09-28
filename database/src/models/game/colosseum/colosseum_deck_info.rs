use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ColosseumDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Position")]
    pub position: i32,
    #[sqlx(rename = "CharInvenIndex")]
    pub char_inven_index: i64,
    #[sqlx(rename = "Sequence")]
    pub sequence: Option<i32>,
    #[sqlx(rename = "CostumeInvenIndex")]
    pub costume_inven_index: Option<i64>,
}
