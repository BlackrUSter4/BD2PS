use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ActionPlayerStateInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Type")]
    pub r#type: Option<i32>,
    #[sqlx(rename = "Seq")]
    pub seq: Option<i32>,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "PlayerPositionIndex")]
    pub player_position_index: Option<i64>,
    #[sqlx(rename = "PlayerVectorIndex")]
    pub player_vector_index: Option<i64>,
    #[sqlx(rename = "KnockbackInfoIndex")]
    pub knockback_info_index: Option<i64>,
    #[sqlx(rename = "Speed")]
    pub speed: Option<f32>,
    #[sqlx(rename = "DeltaTime")]
    pub delta_time: Option<f32>,
    #[sqlx(rename = "SendTime")]
    pub send_time: Option<i64>,
    #[sqlx(rename = "Health")]
    pub health: Option<i32>,
    #[sqlx(rename = "Stamina")]
    pub stamina: Option<i32>,
    #[sqlx(rename = "RecoveryCount")]
    pub recovery_count: Option<i32>,
    #[sqlx(rename = "SkillId")]
    pub skill_id: Option<i32>,
    #[sqlx(rename = "HitInfoIndex")]
    pub hit_info_index: Option<i64>,
    #[sqlx(rename = "BuffInfosIndex")]
    pub buff_infos_index: Option<String>,
}