use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PlayerHitInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "AttackCharId")]
    pub attack_char_id: Option<i32>,
    #[sqlx(rename = "AttackSkillId")]
    pub attack_skill_id: Option<i32>,
    #[sqlx(rename = "ContactPointIndex")]
    pub contact_point_index: Option<i64>,
}