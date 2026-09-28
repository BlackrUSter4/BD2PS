use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct IdCardItemInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "X")]
    pub x: Option<i32>,
    #[sqlx(rename = "Y")]
    pub y: Option<i32>,
    #[sqlx(rename = "Rotate")]
    pub rotate: Option<f32>,
    #[sqlx(rename = "Scale")]
    pub scale: Option<f32>,
    #[sqlx(rename = "Layer")]
    pub layer: Option<i32>,
    #[sqlx(rename = "Color")]
    pub color: Option<String>,
}