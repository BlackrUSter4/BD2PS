use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FieldTrapInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "PackId")]
    pub pack_id: Option<i32>,
    #[sqlx(rename = "MapId")]
    pub map_id: Option<i32>,
    #[sqlx(rename = "TrapId")]
    pub trap_id: Option<i32>,
    #[sqlx(rename = "State")]
    pub state: Option<i32>,
    #[sqlx(rename = "SwitchObjectId")]
    pub switch_object_id: i32,
}