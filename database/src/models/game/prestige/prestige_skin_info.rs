use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PrestigeSkinInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CostumeId")]
    pub costume_id: Option<i32>,
    #[sqlx(rename = "CostumeDesignId")]
    pub costume_design_id: Option<i32>,
    #[sqlx(rename = "IsSet")]
    pub is_set: Option<bool>,
}