use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct KnockbackInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "KnockbackDirIndex")]
    pub knockback_dir_index: Option<i64>,
    #[sqlx(rename = "KnockbackValue")]
    pub knockback_value: Option<f32>,
    #[sqlx(rename = "KnockbackSpeed")]
    pub knockback_speed: Option<f32>,
    #[sqlx(rename = "IsGuard")]
    pub is_guard: Option<i32>,
}