use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GachaTotalCountInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GachaLogType")]
    pub gacha_log_type: Option<serde_json::Value>,
    #[sqlx(rename = "Char5PickUpCostume")]
    pub char5_pick_up_costume: Option<i32>,
    #[sqlx(rename = "Char5Costume")]
    pub char5_costume: Option<i32>,
    #[sqlx(rename = "Char4Costume")]
    pub char4_costume: Option<i32>,
    #[sqlx(rename = "Char3Costume")]
    pub char3_costume: Option<i32>,
    #[sqlx(rename = "Char5PickUpEquip4")]
    pub char5_pick_up_equip4: Option<i32>,
    #[sqlx(rename = "Char5Equip4")]
    pub char5_equip4: Option<i32>,
    #[sqlx(rename = "Char5Equip3")]
    pub char5_equip3: Option<i32>,
    #[sqlx(rename = "Char4Equip4")]
    pub char4_equip4: Option<i32>,
    #[sqlx(rename = "Char4Equip3")]
    pub char4_equip3: Option<i32>,
    #[sqlx(rename = "Char4Equip2")]
    pub char4_equip2: Option<i32>,
    #[sqlx(rename = "Char3Equip4")]
    pub char3_equip4: Option<i32>,
    #[sqlx(rename = "Char3Equip3")]
    pub char3_equip3: Option<i32>,
    #[sqlx(rename = "Char3Equip2")]
    pub char3_equip2: Option<i32>,
}