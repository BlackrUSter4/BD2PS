use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct HuntDispatchInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "HuntDispatchGroupId")]
    pub hunt_dispatch_group_id: Option<i32>,
    #[sqlx(rename = "HuntDispatchId")]
    pub hunt_dispatch_id: Option<i32>,
    #[sqlx(rename = "Count")]
    pub count: Option<i32>,
    #[sqlx(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[sqlx(rename = "EndTime")]
    pub end_time: Option<i64>,
    #[sqlx(rename = "DecreaseFreeApCount")]
    pub decrease_free_ap_count: Option<i32>,
    #[sqlx(rename = "DecreaseCashApCount")]
    pub decrease_cash_ap_count: Option<i32>,
}