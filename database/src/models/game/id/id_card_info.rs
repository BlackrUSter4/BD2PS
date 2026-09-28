use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct IdCardInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BackgroundIndex")]
    pub background_index: Option<i64>,
    #[sqlx(rename = "SubBackgroundIndex")]
    pub sub_background_index: Option<i64>,
    #[sqlx(rename = "BackgroundEffectIndex")]
    pub background_effect_index: Option<i64>,
    #[sqlx(rename = "StickersIndex")]
    pub stickers_index: Option<String>,
    #[sqlx(rename = "MyInfoIndex")]
    pub my_info_index: Option<i64>,
    #[sqlx(rename = "Rotate")]
    pub rotate: Option<i32>,
}