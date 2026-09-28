use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GachaInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ScheduleInfoIndex")]
    pub schedule_info_index: Option<String>,
    #[sqlx(rename = "GachaUserInfoIndex")]
    pub gacha_user_info_index: Option<String>,
    #[sqlx(rename = "ScheduleEndExchangePoint")]
    pub schedule_end_exchange_point: Option<i32>,
    #[sqlx(rename = "GachaFixedInfoIndex")]
    pub gacha_fixed_info_index: Option<String>,
    #[sqlx(rename = "GachaSelectionInfoIndex")]
    pub gacha_selection_info_index: Option<String>,
    #[sqlx(rename = "GachaSelectionChangeCountInfoIndex")]
    pub gacha_selection_change_count_info_index: Option<String>,
    #[sqlx(rename = "StepUpScheduleInfoIndex")]
    pub step_up_schedule_info_index: Option<String>,
    #[sqlx(rename = "StepUpUserInfoIndex")]
    pub step_up_user_info_index: Option<String>,
    #[sqlx(rename = "ResemaraPreviewItemInfoIndex")]
    pub resemara_preview_item_info_index: Option<String>,
}