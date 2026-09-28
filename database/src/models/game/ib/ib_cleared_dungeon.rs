use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct IbClearedDungeon {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Season")]
    pub season: i32,
    #[sqlx(rename = "DungeonId")]
    pub dungeon_id: i32,
}
