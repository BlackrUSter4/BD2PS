use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ChargeCostInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CostTimeInfoIndex")]
    pub cost_time_info_index: Option<i32>,
    #[sqlx(rename = "EventScheduleInfoIndex")]
    pub event_schedule_info_index: Option<i32>,
}
