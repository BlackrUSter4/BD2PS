use crate::models::game::guild::guild_raid_flag_grade_version_info::GuildRaidFlagGradeVersionInfo;
use sqlx::SqlitePool;

/// Add a single GuildRaidFlagGradeVersionInfo record from a Rust struct.
pub async fn add_guild_raid_flag_grade_version_info(
    pool: &SqlitePool,
    data: &GuildRaidFlagGradeVersionInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidFlagGradeVersionInfo (
    Uid,
    GroupId,
    MinSeason,
    MaxSeason
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.min_season)
    .bind(&data.max_season)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_flag_grade_version_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidFlagGradeVersionInfo>> {
    sqlx::query_as::<_, GuildRaidFlagGradeVersionInfo>(
        "SELECT * FROM GuildRaidFlagGradeVersionInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all GuildRaidFlagGradeVersionInfo rows for a UID.
pub async fn delete_guild_raid_flag_grade_version_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidFlagGradeVersionInfo WHERE Uid = ?")
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
) -> sqlx::Result<GuildRaidFlagGradeVersionInfo> {
    sqlx::query_as::<_, GuildRaidFlagGradeVersionInfo>(
        "SELECT * FROM GuildRaidFlagGradeVersionInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GuildRaidFlagGradeVersionInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildRaidFlagGradeVersionInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildRaidFlagGradeVersionInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildRaidFlagGradeVersionInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildRaidFlagGradeVersionInfo (
    Uid,
    GroupId,
    MinSeason,
    MaxSeason
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.min_season)
    .bind(&data.max_season)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
