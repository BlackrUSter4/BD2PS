use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FieldObjectRespawnInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "FieldObjectGroupId")]
    pub field_object_group_id: Option<i32>,
    #[sqlx(rename = "RespawnTime")]
    pub respawn_time: Option<i64>,
    /// Which pack `field_object_group_id` belongs to — see migration 412.
    #[sqlx(rename = "PackId")]
    pub pack_id: i32,
}