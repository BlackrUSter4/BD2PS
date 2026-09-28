use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterHuntUserInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Season")]
    pub season: Option<i32>,
    #[sqlx(rename = "MonsterHuntId")]
    pub monster_hunt_id: Option<i32>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "StartHp")]
    pub start_hp: Option<i64>,
    #[sqlx(rename = "HighestFirstTurnDamage")]
    pub highest_first_turn_damage: Option<i32>,
    #[sqlx(rename = "HighestHp")]
    pub highest_hp: Option<i64>,
    #[sqlx(rename = "HighestHpDate")]
    pub highest_hp_date: Option<i64>,
    #[sqlx(rename = "CurrentLevelHighestDamage")]
    pub current_level_highest_damage: Option<i64>,
    #[sqlx(rename = "DailyHighestDamage")]
    pub daily_highest_damage: Option<i64>,
    #[sqlx(rename = "SeasonReward")]
    pub season_reward: Option<bool>,
    #[sqlx(rename = "DailyRewardLevel")]
    pub daily_reward_level: Option<i32>,
    #[sqlx(rename = "DailyRewardDate")]
    pub daily_reward_date: Option<i64>,
}