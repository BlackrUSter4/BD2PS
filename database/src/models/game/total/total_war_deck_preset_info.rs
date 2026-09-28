use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct TotalWarDeckPresetInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Slot")]
    pub slot: Option<i32>,
    #[sqlx(rename = "PresetName")]
    pub preset_name: Option<String>,
    #[sqlx(rename = "ResourceId")]
    pub resource_id: Option<i32>,
    #[sqlx(rename = "ResourceColor")]
    pub resource_color: Option<i32>,
    #[sqlx(rename = "DeckInfoIndex")]
    pub deck_info_index: Option<String>,
}
