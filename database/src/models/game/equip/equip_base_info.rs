use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EquipBaseInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "MainOptionIndex")]
    pub main_option_index: Option<String>,
    #[sqlx(rename = "SubOptionIndex")]
    pub sub_option_index: Option<String>,
    #[sqlx(rename = "PrivateOptionIndex")]
    pub private_option_index: Option<i64>,
    #[sqlx(rename = "Rank")]
    pub rank: i32,
    #[sqlx(rename = "PrevMainOptionIndex")]
    pub prev_main_option_index: Option<String>,
    #[sqlx(rename = "PrevSubOptionIndex")]
    pub prev_sub_option_index: Option<String>,
}