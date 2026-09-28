use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpUserInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Vp")]
    pub vp: i32,
    #[sqlx(rename = "WinCount")]
    pub win_count: i32,
    #[sqlx(rename = "LoseCount")]
    pub lose_count: i32,
    #[sqlx(rename = "SeasonAttackWinCount")]
    pub season_attack_win_count: i32,
    #[sqlx(rename = "SeasonAttackLoseCount")]
    pub season_attack_lose_count: i32,
    #[sqlx(rename = "SeasonDefenseWinCount")]
    pub season_defense_win_count: i32,
    #[sqlx(rename = "SeasonDefenseLoseCount")]
    pub season_defense_lose_count: i32,
    #[sqlx(rename = "PrevSeasonAttackWinCount")]
    pub prev_season_attack_win_count: i32,
    #[sqlx(rename = "PrevSeasonAttackLoseCount")]
    pub prev_season_attack_lose_count: i32,
    #[sqlx(rename = "PrevSeasonDefenseWinCount")]
    pub prev_season_defense_win_count: i32,
    #[sqlx(rename = "PrevSeasonDefenseLoseCount")]
    pub prev_season_defense_lose_count: i32,
    #[sqlx(rename = "DeckSeasonAttackWinCount")]
    pub deck_season_attack_win_count: i32,
    #[sqlx(rename = "DeckSeasonAttackLoseCount")]
    pub deck_season_attack_lose_count: i32,
    #[sqlx(rename = "DeckSeasonDefenseWinCount")]
    pub deck_season_defense_win_count: i32,
    #[sqlx(rename = "DeckSeasonDefenseLoseCount")]
    pub deck_season_defense_lose_count: i32,
    #[sqlx(rename = "DeckSeasonAttackResetTime")]
    pub deck_season_attack_reset_time: Option<i64>,
    #[sqlx(rename = "DeckSeasonDefenseResetTime")]
    pub deck_season_defense_reset_time: Option<i64>,
    #[sqlx(rename = "Season")]
    pub season: i32,
    #[sqlx(rename = "CurrentBattleEnemyIndex")]
    pub current_battle_enemy_index: Option<i64>,
    #[sqlx(rename = "OnceRewardClaimed")]
    pub once_reward_claimed: String,
    #[sqlx(rename = "SeasonRewardClaimedSeason")]
    pub season_reward_claimed_season: i32,
}

impl Default for PvpUserInfo {
    fn default() -> Self {
        Self {
            uid: 0,
            vp: 1000,
            win_count: 0,
            lose_count: 0,
            season_attack_win_count: 0,
            season_attack_lose_count: 0,
            season_defense_win_count: 0,
            season_defense_lose_count: 0,
            prev_season_attack_win_count: 0,
            prev_season_attack_lose_count: 0,
            prev_season_defense_win_count: 0,
            prev_season_defense_lose_count: 0,
            deck_season_attack_win_count: 0,
            deck_season_attack_lose_count: 0,
            deck_season_defense_win_count: 0,
            deck_season_defense_lose_count: 0,
            deck_season_attack_reset_time: None,
            deck_season_defense_reset_time: None,
            season: 1,
            current_battle_enemy_index: None,
            once_reward_claimed: String::new(),
            season_reward_claimed_season: 0,
        }
    }
}
