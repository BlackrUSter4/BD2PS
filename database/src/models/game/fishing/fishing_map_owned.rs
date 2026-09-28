use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FishingMapOwned {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "MapId")]
    pub map_id: i32,
}
