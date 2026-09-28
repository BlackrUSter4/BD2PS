use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GachaUserInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "Point")]
    pub point: Option<i32>,
    #[sqlx(rename = "TotalBuyCount")]
    pub total_buy_count: Option<i32>,
    #[sqlx(rename = "OneFreePickCount")]
    pub one_free_pick_count: Option<i32>,
    #[sqlx(rename = "OneCashPickCount")]
    pub one_cash_pick_count: Option<i32>,
    #[sqlx(rename = "TenFreePickCount")]
    pub ten_free_pick_count: Option<i32>,
    #[sqlx(rename = "TenCashPickCount")]
    pub ten_cash_pick_count: Option<i32>,
    #[sqlx(rename = "ExchangeItemCount")]
    pub exchange_item_count: Option<i32>,
    #[sqlx(rename = "ExchangeMileageCount")]
    pub exchange_mileage_count: Option<i32>,
}