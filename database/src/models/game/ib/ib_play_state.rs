use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct IbPlayState {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "DungeonId")]
    pub dungeon_id: i32,
    #[sqlx(rename = "StageId")]
    pub stage_id: i32,
    #[sqlx(rename = "Life")]
    pub life: i32,
    #[sqlx(rename = "Coin")]
    pub coin: i32,
    #[sqlx(rename = "Season")]
    pub season: i32,
    #[sqlx(rename = "ShopReloadCount")]
    pub shop_reload_count: i32,
    #[sqlx(rename = "IsStageEnter")]
    pub is_stage_enter: bool,
    #[sqlx(rename = "CurrentBattleMode")]
    pub current_battle_mode: Option<i32>,
    #[sqlx(rename = "CurrentRandomSeed")]
    pub current_random_seed: Option<i32>,
}

impl Default for IbPlayState {
    fn default() -> Self {
        Self {
            uid: 0,
            dungeon_id: 0,
            stage_id: 0,
            life: 3,
            coin: 0,
            season: 1,
            shop_reload_count: 0,
            is_stage_enter: false,
            current_battle_mode: None,
            current_random_seed: None,
        }
    }
}
