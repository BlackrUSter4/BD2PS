use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleRogueLikeScoreInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ScoreItemInfoIndex")]
    pub score_item_info_index: Option<String>,
    #[sqlx(rename = "TotalScore")]
    pub total_score: Option<i32>,
    #[sqlx(rename = "Obsidian")]
    pub obsidian: Option<i32>,
    #[sqlx(rename = "AllUserTotalScore")]
    pub all_user_total_score: Option<i64>,
    #[sqlx(rename = "MaxTryLevel")]
    pub max_try_level: Option<i32>,
    #[sqlx(rename = "MaxRewardLevel")]
    pub max_reward_level: Option<i32>,
    #[sqlx(rename = "CrystalDamage")]
    pub crystal_damage: Option<i64>,
}