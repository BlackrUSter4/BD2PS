use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::guild::guild_raid_main_info::GuildRaidMainInfo;
/// Insert a full JSON array of GuildRaidMainInfo records for a UID.
pub async fn insert_guild_raid_main_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("guildRaidMainInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_guild_raid_main_info: missing or invalid 'guildRaidMainInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated primitive fields - insert one row per value
        let play_day_array = entry.get("playDay").and_then(|v| v.as_array());
        if let Some(values) = play_day_array {
            for item in values {
                let season = entry
                    .get("season")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let raid_day = entry
                    .get("raidDay")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let today_normal_battle_count = entry
                    .get("todayNormalBattleCount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let user_score = entry
                    .get("userScore")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                let last_score_reward_id = entry
                    .get("lastScoreRewardId")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let guild_total_score = entry
                    .get("guildTotalScore")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                let guild_top_percent = entry
                    .get("guildTopPercent")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                let golem_level = entry
                    .get("golemLevel")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let golem_exp = entry
                    .get("golemExp")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let obtainable_season_reward = entry
                    .get("obtainableSeasonReward")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let today_supporter_use_count = entry
                    .get("todaySupporterUseCount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let total_supporter_rental_count = entry
                    .get("totalSupporterRentalCount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let top_guild_score = entry
                    .get("topGuildScore")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                let schedule_history_info_index = entry
                    .get("scheduleHistoryInfoIndex")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let flag_grade_version_info_index = entry
                    .get("flagGradeVersionInfoIndex")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let guild_rank = entry
                    .get("guildRank")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let play_day = item.as_i64().unwrap_or_default() as i32;

                sqlx::query(
                    r#"
INSERT INTO GuildRaidMainInfo (
    Uid,
    Season,
    RaidDay,
    TodayNormalBattleCount,
    UserScore,
    LastScoreRewardId,
    GuildTotalScore,
    GuildTopPercent,
    GolemLevel,
    GolemExp,
    ObtainableSeasonReward,
    TodaySupporterUseCount,
    TotalSupporterRentalCount,
    TopGuildScore,
    ScheduleHistoryInfoIndex,
    FlagGradeVersionInfoIndex,
    GuildRank,
    PlayDay
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&season)
                .bind(&raid_day)
                .bind(&today_normal_battle_count)
                .bind(&user_score)
                .bind(&last_score_reward_id)
                .bind(&guild_total_score)
                .bind(&guild_top_percent)
                .bind(&golem_level)
                .bind(&golem_exp)
                .bind(&obtainable_season_reward)
                .bind(&today_supporter_use_count)
                .bind(&total_supporter_rental_count)
                .bind(&top_guild_score)
                .bind(&schedule_history_info_index)
                .bind(&flag_grade_version_info_index)
                .bind(&guild_rank)
                .bind(&play_day)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Add a single GuildRaidMainInfo record from a Rust struct.
pub async fn add_guild_raid_main_info(pool: &SqlitePool, data: &GuildRaidMainInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidMainInfo (
    Uid,
    Season,
    RaidDay,
    TodayNormalBattleCount,
    UserScore,
    LastScoreRewardId,
    GuildTotalScore,
    GuildTopPercent,
    GolemLevel,
    GolemExp,
    ObtainableSeasonReward,
    TodaySupporterUseCount,
    TotalSupporterRentalCount,
    TopGuildScore,
    PlayDay,
    ScheduleHistoryInfoIndex,
    FlagGradeVersionInfoIndex,
    GuildRank
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.season)
    .bind(&data.raid_day)
    .bind(&data.today_normal_battle_count)
    .bind(&data.user_score)
    .bind(&data.last_score_reward_id)
    .bind(&data.guild_total_score)
    .bind(&data.guild_top_percent)
    .bind(&data.golem_level)
    .bind(&data.golem_exp)
    .bind(&data.obtainable_season_reward)
    .bind(&data.today_supporter_use_count)
    .bind(&data.total_supporter_rental_count)
    .bind(&data.top_guild_score)
    .bind(&data.play_day)
    .bind(&data.schedule_history_info_index)
    .bind(&data.flag_grade_version_info_index)
    .bind(&data.guild_rank)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_main_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GuildRaidMainInfo>> {
    sqlx::query_as::<_, GuildRaidMainInfo>("SELECT * FROM GuildRaidMainInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildRaidMainInfo rows for a UID.
pub async fn delete_guild_raid_main_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidMainInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}