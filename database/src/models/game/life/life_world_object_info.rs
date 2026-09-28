use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct LifeWorldObjectInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ChunkId")]
    pub chunk_id: i32,
    #[sqlx(rename = "ObjectIndex")]
    pub object_index: Option<i32>,
    #[sqlx(rename = "ObjectId")]
    pub object_id: Option<i32>,
    #[sqlx(rename = "X")]
    pub x: Option<i32>,
    #[sqlx(rename = "Y")]
    pub y: Option<i32>,
    #[sqlx(rename = "Rotate")]
    pub rotate: Option<i32>,
    #[sqlx(rename = "Status")]
    pub status: Option<i32>,
    #[sqlx(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[sqlx(rename = "EndTime")]
    pub end_time: Option<i64>,
    #[sqlx(rename = "ParentIndex")]
    pub parent_index: Option<i64>,
}
