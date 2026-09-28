use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CostTimeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "LastChargeTime")]
    pub last_charge_time: Option<i64>,
    #[sqlx(rename = "CostItemInfoIndex")]
    pub cost_item_info_index: Option<i64>,
}