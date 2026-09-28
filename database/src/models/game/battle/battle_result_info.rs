use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct BattleResultInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BattleResult")]
    pub battle_result: Option<i32>,
    #[sqlx(rename = "RedCharInfoIndex")]
    pub red_char_info_index: Option<String>,
    #[sqlx(rename = "BlueCharInfoIndex")]
    pub blue_char_info_index: Option<String>,
    #[sqlx(rename = "GridItemIndex")]
    pub grid_item_index: Option<String>,
    #[sqlx(rename = "BattleStatisticsInfoIndex")]
    pub battle_statistics_info_index: Option<i64>,
    #[sqlx(rename = "GolemInfoIndex")]
    pub golem_info_index: Option<i64>,
    #[sqlx(rename = "RedDeckOutCharInfoIndex")]
    pub red_deck_out_char_info_index: Option<String>,
    #[sqlx(rename = "BlueDeckOutCharInfoIndex")]
    pub blue_deck_out_char_info_index: Option<String>,
}
