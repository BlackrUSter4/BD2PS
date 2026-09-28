use crate::models::game::guild::guild_raid_normal_battle_history_info::GuildRaidNormalBattleHistoryInfo;
use sqlx::SqlitePool;

/// Add a single GuildRaidNormalBattleHistoryInfo record from a Rust struct.
pub async fn add_guild_raid_normal_battle_history_info(
    pool: &SqlitePool,
    data: &GuildRaidNormalBattleHistoryInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidNormalBattleHistoryInfo (
    Uid,
    OwnerIndex,
    UserId,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    TitleId
) VALUES (
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
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.title_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_normal_battle_history_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidNormalBattleHistoryInfo>> {
    sqlx::query_as::<_, GuildRaidNormalBattleHistoryInfo>(
        "SELECT * FROM GuildRaidNormalBattleHistoryInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all GuildRaidNormalBattleHistoryInfo rows for a UID.
pub async fn delete_guild_raid_normal_battle_history_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidNormalBattleHistoryInfo WHERE Uid = ?")
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
) -> sqlx::Result<GuildRaidNormalBattleHistoryInfo> {
    sqlx::query_as::<_, GuildRaidNormalBattleHistoryInfo>(
        "SELECT * FROM GuildRaidNormalBattleHistoryInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GuildRaidNormalBattleHistoryInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildRaidNormalBattleHistoryInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildRaidNormalBattleHistoryInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(
    pool: &SqlitePool,
    data: &GuildRaidNormalBattleHistoryInfo,
) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildRaidNormalBattleHistoryInfo (
    Uid,
    OwnerIndex,
    UserId,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    TitleId
) VALUES (
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
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.title_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
