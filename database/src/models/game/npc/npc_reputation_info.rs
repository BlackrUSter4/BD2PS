use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct NpcReputationInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "PackId")]
    pub pack_id: Option<i32>,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "NpcId")]
    pub npc_id: Option<i32>,
    #[sqlx(rename = "Point")]
    pub point: Option<i32>,
}