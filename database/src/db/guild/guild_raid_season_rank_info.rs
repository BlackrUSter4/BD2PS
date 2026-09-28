use crate::models::game::guild::guild_raid_season_rank_info::GuildRaidSeasonRankInfo;
use sqlx::SqlitePool;

/// Add a single GuildRaidSeasonRankInfo record from a Rust struct.
pub async fn add_guild_raid_season_rank_info(
    pool: &SqlitePool,
    data: &GuildRaidSeasonRankInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidSeasonRankInfo (
    Uid,
    Rank,
    Score,
    GuildIndex,
    GuildName,
    Message,
    Icon,
    IconColor,
    FlagGrade,
    MemberCount,
    OverKillDamage
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.score)
    .bind(&data.guild_index)
    .bind(&data.guild_name)
    .bind(&data.message)
    .bind(&data.icon)
    .bind(&data.icon_color)
    .bind(&data.flag_grade)
    .bind(&data.member_count)
    .bind(&data.over_kill_damage)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_season_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidSeasonRankInfo>> {
    sqlx::query_as::<_, GuildRaidSeasonRankInfo>(
        "SELECT * FROM GuildRaidSeasonRankInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all GuildRaidSeasonRankInfo rows for a UID.
pub async fn delete_guild_raid_season_rank_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidSeasonRankInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<GuildRaidSeasonRankInfo> {
    sqlx::query_as::<_, GuildRaidSeasonRankInfo>(
        "SELECT * FROM GuildRaidSeasonRankInfo WHERE Uid = ? AND Index = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<GuildRaidSeasonRankInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildRaidSeasonRankInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildRaidSeasonRankInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildRaidSeasonRankInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildRaidSeasonRankInfo (
    Uid,
    Rank,
    Score,
    GuildIndex,
    GuildName,
    Message,
    Icon,
    IconColor,
    FlagGrade,
    MemberCount,
    OverKillDamage
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.score)
    .bind(&data.guild_index)
    .bind(&data.guild_name)
    .bind(&data.message)
    .bind(&data.icon)
    .bind(&data.icon_color)
    .bind(&data.flag_grade)
    .bind(&data.member_count)
    .bind(&data.over_kill_damage)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
