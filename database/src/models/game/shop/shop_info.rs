use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ShopInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ShopId")]
    pub shop_id: i32,
    #[sqlx(rename = "ShopRemainTime")]
    pub shop_remain_time: Option<i32>,
    #[sqlx(rename = "ShopRandSeed")]
    pub shop_rand_seed: Option<i32>,
}
