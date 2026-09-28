use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct LifeShopResetInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "DailyResetTime")]
    pub daily_reset_time: Option<i64>,
    #[sqlx(rename = "WeeklyResetTime")]
    pub weekly_reset_time: Option<i64>,
    #[sqlx(rename = "MonthlyResetTime")]
    pub monthly_reset_time: Option<i64>,
}
