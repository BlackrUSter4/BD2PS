use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterHuntPresetInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "PresetInfoIndex")]
    pub preset_info_index: Option<String>,
    #[sqlx(rename = "Slot")]
    pub slot: Option<i32>,
    #[sqlx(rename = "PresetName")]
    pub preset_name: Option<String>,
    #[sqlx(rename = "PresetResourceId")]
    pub preset_resource_id: Option<i32>,
    #[sqlx(rename = "PresetResourceColor")]
    pub preset_resource_color: Option<i32>,
}