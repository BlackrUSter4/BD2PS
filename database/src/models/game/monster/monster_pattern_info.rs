use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterPatternInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "TargetOwnerIndex")]
    pub target_owner_index: Option<i64>,
    #[sqlx(rename = "PatternId")]
    pub pattern_id: Option<i32>,
    #[sqlx(rename = "DistanceToAttack")]
    pub distance_to_attack: Option<f32>,
}