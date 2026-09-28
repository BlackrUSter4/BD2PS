use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ShopProductInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ShopId")]
    pub shop_id: i32,
    #[sqlx(rename = "ProductId")]
    pub product_id: i32,
    #[sqlx(rename = "BuyCount")]
    pub buy_count: i32,
}
