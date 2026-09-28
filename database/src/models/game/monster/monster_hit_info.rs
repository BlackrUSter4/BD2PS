use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterHitInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "RageValue")]
    pub rage_value: Option<i32>,
    #[sqlx(rename = "GroggyValue")]
    pub groggy_value: Option<i32>,
    #[sqlx(rename = "TargetPartsId")]
    pub target_parts_id: Option<i32>,
    #[sqlx(rename = "TargetAttackType")]
    pub target_attack_type: Option<i32>,
    #[sqlx(rename = "Damage")]
    pub damage: Option<i32>,
    #[sqlx(rename = "AttackOwnerIndex")]
    pub attack_owner_index: Option<i64>,
    #[sqlx(rename = "ContactPointIndex")]
    pub contact_point_index: Option<i64>,
    #[sqlx(rename = "DisplayAttackCount")]
    pub display_attack_count: Option<i32>,
    #[sqlx(rename = "IsCritical")]
    pub is_critical: Option<i32>,
    #[sqlx(rename = "IsWeak")]
    pub is_weak: Option<i32>,
}