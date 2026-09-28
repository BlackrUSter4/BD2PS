use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct IbShop {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Slot")]
    pub slot: i32,
    #[sqlx(rename = "Type")]
    pub r#type: i32,
    #[sqlx(rename = "ItemId")]
    pub item_id: i32,
    #[sqlx(rename = "Price")]
    pub price: i32,
    #[sqlx(rename = "OriginalPrice")]
    pub original_price: i32,
    #[sqlx(rename = "IsDiscount")]
    pub is_discount: bool,
    #[sqlx(rename = "IsReserved")]
    pub is_reserved: bool,
    #[sqlx(rename = "IsSoldOut")]
    pub is_sold_out: bool,
}
