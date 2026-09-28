use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FieldObjectPositionInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "MapId")]
    pub map_id: Option<i32>,
    #[sqlx(rename = "X")]
    pub x: Option<f32>,
    #[sqlx(rename = "Y")]
    pub y: Option<f32>,
    #[sqlx(rename = "Z")]
    pub z: Option<f32>,
}