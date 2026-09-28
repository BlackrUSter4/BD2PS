use crate::models::game::guild::guild_raid_member_rank_info::GuildRaidMemberRankInfo;
use sqlx::SqlitePool;

/// Add a single GuildRaidMemberRankInfo record from a Rust struct.
pub async fn add_guild_raid_member_rank_info(
    pool: &SqlitePool,
    data: &GuildRaidMemberRankInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidMemberRankInfo (
    Uid,
    Rank,
    OwnerIndex,
    UserId,
    Score,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    TitleId,
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.score)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.title_id)
    .bind(&data.over_kill_damage)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_member_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidMemberRankInfo>> {
    sqlx::query_as::<_, GuildRaidMemberRankInfo>(
        "SELECT * FROM GuildRaidMemberRankInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all GuildRaidMemberRankInfo rows for a UID.
pub async fn delete_guild_raid_member_rank_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidMemberRankInfo WHERE Uid = ?")
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
) -> sqlx::Result<GuildRaidMemberRankInfo> {
    sqlx::query_as::<_, GuildRaidMemberRankInfo>(
        "SELECT * FROM GuildRaidMemberRankInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GuildRaidMemberRankInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildRaidMemberRankInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildRaidMemberRankInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildRaidMemberRankInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildRaidMemberRankInfo (
    Uid,
    Rank,
    OwnerIndex,
    UserId,
    Score,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    TitleId,
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.score)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.title_id)
    .bind(&data.over_kill_damage)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
