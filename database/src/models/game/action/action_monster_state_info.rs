use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ActionMonsterStateInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Type")]
    pub r#type: Option<i32>,
    #[sqlx(rename = "Seq")]
    pub seq: Option<i32>,
    #[sqlx(rename = "MonsterId")]
    pub monster_id: Option<i32>,
    #[sqlx(rename = "MonsterIndex")]
    pub monster_index: Option<i32>,
    #[sqlx(rename = "MonsterPositionIndex")]
    pub monster_position_index: Option<i64>,
    #[sqlx(rename = "MonsterVectorIndex")]
    pub monster_vector_index: Option<i64>,
    #[sqlx(rename = "Speed")]
    pub speed: Option<f32>,
    #[sqlx(rename = "DeltaTime")]
    pub delta_time: Option<f32>,
    #[sqlx(rename = "SendTime")]
    pub send_time: Option<i64>,
    #[sqlx(rename = "Health")]
    pub health: Option<i32>,
    #[sqlx(rename = "RageValue")]
    pub rage_value: Option<i32>,
    #[sqlx(rename = "GroggyValue")]
    pub groggy_value: Option<i32>,
    #[sqlx(rename = "State")]
    pub state: Option<i32>,
    #[sqlx(rename = "PatternInfoIndex")]
    pub pattern_info_index: Option<i64>,
    #[sqlx(rename = "HitInfoIndex")]
    pub hit_info_index: Option<i64>,
    #[sqlx(rename = "PartsInfoIndex")]
    pub parts_info_index: Option<String>,
    #[sqlx(rename = "BuffInfosIndex")]
    pub buff_infos_index: Option<String>,
    #[sqlx(rename = "AttackSkillId")]
    pub attack_skill_id: Option<i32>,
}