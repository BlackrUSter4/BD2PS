use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::guild::guild_raid_boss_battle_info::GuildRaidBossBattleInfo;
/// Insert a full JSON array of GuildRaidBossBattleInfo records for a UID.
pub async fn insert_guild_raid_boss_battle_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("guildRaidBossBattleInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_guild_raid_boss_battle_info: missing or invalid 'guildRaidBossBattleInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let guild_total_score = entry
            .get("guildTotalScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let guild_top_percent = entry
            .get("guildTopPercent")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let highest_level = entry
            .get("highestLevel")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let highest_score = entry
            .get("highestScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let top_member_owner_index = entry
            .get("topMemberOwnerIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let top_member_user_id = entry
            .get("topMemberUserId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let top_member_score = entry
            .get("topMemberScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO GuildRaidBossBattleInfo (
    Uid,
    GuildTotalScore,
    GuildTopPercent,
    HighestLevel,
    HighestScore,
    TopMemberOwnerIndex,
    TopMemberUserId,
    TopMemberScore
) VALUES (
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
        .bind(&guild_total_score)
        .bind(&guild_top_percent)
        .bind(&highest_level)
        .bind(&highest_score)
        .bind(&top_member_owner_index)
        .bind(&top_member_user_id)
        .bind(&top_member_score)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single GuildRaidBossBattleInfo record from a Rust struct.
pub async fn add_guild_raid_boss_battle_info(pool: &SqlitePool, data: &GuildRaidBossBattleInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidBossBattleInfo (
    Uid,
    GuildTotalScore,
    GuildTopPercent,
    HighestLevel,
    HighestScore,
    TopMemberOwnerIndex,
    TopMemberUserId,
    TopMemberScore
) VALUES (
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
    .bind(&data.guild_total_score)
    .bind(&data.guild_top_percent)
    .bind(&data.highest_level)
    .bind(&data.highest_score)
    .bind(&data.top_member_owner_index)
    .bind(&data.top_member_user_id)
    .bind(&data.top_member_score)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_boss_battle_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GuildRaidBossBattleInfo>> {
    sqlx::query_as::<_, GuildRaidBossBattleInfo>("SELECT * FROM GuildRaidBossBattleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildRaidBossBattleInfo rows for a UID.
pub async fn delete_guild_raid_boss_battle_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidBossBattleInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}