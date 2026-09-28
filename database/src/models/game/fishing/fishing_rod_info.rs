use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FishingRodInfo {
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "RodId")]
    pub rod_id: i32,
    #[sqlx(rename = "TimeValue")]
    pub time_value: Option<i64>,
}
