use crate::models::game::guild::guild_raid_play_info::GuildRaidPlayInfo;
use sqlx::SqlitePool;

/// Add a single GuildRaidPlayInfo record from a Rust struct.
pub async fn add_guild_raid_play_info(
    pool: &SqlitePool,
    data: &GuildRaidPlayInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidPlayInfo (
    Uid,
    BossScore,
    TotalScore,
    TopPercent,
    IsPlayRaidToday,
    IsNormalBattlePlay,
    BattleMode,
    Rank
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
"#,
    )
    .bind(&data.uid)
    .bind(&data.boss_score)
    .bind(&data.total_score)
    .bind(&data.top_percent)
    .bind(&data.is_play_raid_today)
    .bind(&data.is_normal_battle_play)
    .bind(&data.battle_mode)
    .bind(&data.rank)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_play_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidPlayInfo>> {
    sqlx::query_as::<_, GuildRaidPlayInfo>("SELECT * FROM GuildRaidPlayInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildRaidPlayInfo rows for a UID.
pub async fn delete_guild_raid_play_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidPlayInfo WHERE Uid = ?")
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
) -> sqlx::Result<GuildRaidPlayInfo> {
    sqlx::query_as::<_, GuildRaidPlayInfo>(
        "SELECT * FROM GuildRaidPlayInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GuildRaidPlayInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildRaidPlayInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildRaidPlayInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildRaidPlayInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildRaidPlayInfo (
    Uid,
    BossScore,
    TotalScore,
    TopPercent,
    IsPlayRaidToday,
    IsNormalBattlePlay,
    BattleMode,
    Rank
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
"#,
    )
    .bind(&data.uid)
    .bind(&data.boss_score)
    .bind(&data.total_score)
    .bind(&data.top_percent)
    .bind(&data.is_play_raid_today)
    .bind(&data.is_normal_battle_play)
    .bind(&data.battle_mode)
    .bind(&data.rank)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
