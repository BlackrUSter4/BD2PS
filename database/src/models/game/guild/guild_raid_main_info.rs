use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidMainInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Season")]
    pub season: Option<i32>,
    #[sqlx(rename = "RaidDay")]
    pub raid_day: Option<i32>,
    #[sqlx(rename = "TodayNormalBattleCount")]
    pub today_normal_battle_count: Option<i32>,
    #[sqlx(rename = "UserScore")]
    pub user_score: Option<i64>,
    #[sqlx(rename = "LastScoreRewardId")]
    pub last_score_reward_id: Option<i32>,
    #[sqlx(rename = "GuildTotalScore")]
    pub guild_total_score: Option<i64>,
    #[sqlx(rename = "GuildTopPercent")]
    pub guild_top_percent: Option<f64>,
    #[sqlx(rename = "GolemLevel")]
    pub golem_level: Option<i32>,
    #[sqlx(rename = "GolemExp")]
    pub golem_exp: Option<i32>,
    #[sqlx(rename = "ObtainableSeasonReward")]
    pub obtainable_season_reward: Option<i32>,
    #[sqlx(rename = "TodaySupporterUseCount")]
    pub today_supporter_use_count: Option<i32>,
    #[sqlx(rename = "TotalSupporterRentalCount")]
    pub total_supporter_rental_count: Option<i32>,
    #[sqlx(rename = "TopGuildScore")]
    pub top_guild_score: Option<i64>,
    #[sqlx(rename = "PlayDay")]
    pub play_day: i32,
    #[sqlx(rename = "ScheduleHistoryInfoIndex")]
    pub schedule_history_info_index: Option<String>,
    #[sqlx(rename = "FlagGradeVersionInfoIndex")]
    pub flag_grade_version_info_index: Option<String>,
    #[sqlx(rename = "GuildRank")]
    pub guild_rank: Option<i32>,
}