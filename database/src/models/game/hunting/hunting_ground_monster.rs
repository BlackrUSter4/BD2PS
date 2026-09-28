use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct HuntingGroundMonster {
    #[sqlx(rename = "HuntingGroundIndex")]
    pub hunting_ground_index: i32,
    #[sqlx(rename = "MonsterIndex")]
    pub monster_index: i32,
}
