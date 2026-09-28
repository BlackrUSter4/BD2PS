use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleRogueLikeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "StateInfoIndex")]
    pub state_info_index: Option<i64>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "FloorInfoIndex")]
    pub floor_info_index: Option<String>,
    #[sqlx(rename = "DeckInfoIndex")]
    pub deck_info_index: Option<String>,
    #[sqlx(rename = "CharInfoIndex")]
    pub char_info_index: Option<String>,
    #[sqlx(rename = "CostumeInfoIndex")]
    pub costume_info_index: Option<String>,
    #[sqlx(rename = "RelicInfoIndex")]
    pub relic_info_index: Option<String>,
    #[sqlx(rename = "ChoiceInfoIndex")]
    pub choice_info_index: Option<i64>,
    #[sqlx(rename = "ReRoll")]
    pub re_roll: Option<i32>,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "GrowthInfoIndex")]
    pub growth_info_index: Option<String>,
    #[sqlx(rename = "EventInfoIndex")]
    pub event_info_index: Option<i64>,
    #[sqlx(rename = "ShopInfoIndex")]
    pub shop_info_index: Option<i64>,
    #[sqlx(rename = "BattleLevel")]
    pub battle_level: Option<i32>,
    #[sqlx(rename = "MaxTryLevel")]
    pub max_try_level: Option<i32>,
    #[sqlx(rename = "Obsidian")]
    pub obsidian: Option<i32>,
    #[sqlx(rename = "RogueLikeGold")]
    pub rogue_like_gold: Option<i32>,
    #[sqlx(rename = "Season")]
    pub season: Option<i32>,
    #[sqlx(rename = "RegularSeason")]
    pub regular_season: Option<i32>,
    #[sqlx(rename = "SeasonReward")]
    pub season_reward: Option<i32>,
    #[sqlx(rename = "MaxRewardLevel")]
    pub max_reward_level: Option<i32>,
    #[sqlx(rename = "HighestCrystalDamage")]
    pub highest_crystal_damage: Option<i64>,
}