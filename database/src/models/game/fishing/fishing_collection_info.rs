use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FishingCollectionInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "FishId")]
    pub fish_id: i32,
    #[sqlx(rename = "MaxSize")]
    pub max_size: i32,
    #[sqlx(rename = "MinSize")]
    pub min_size: i32,
    #[sqlx(rename = "CreateTime")]
    pub create_time: Option<i64>,
}
