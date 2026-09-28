use crate::models::game::guild::guild_info::GuildInfo;
use sqlx::SqlitePool;

/// Add a single GuildInfo record from a Rust struct.
pub async fn add_guild_info(pool: &SqlitePool, data: &GuildInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildInfo (
    Uid,
    GuildBaseInfoIndex,
    JoinType,
    Message,
    UpdateDate,
    Date,
    MemberCount,
    DeleteRemainingTime,
    NoticeUpdateDate
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
    .bind(&data.guild_base_info_index)
    .bind(&data.join_type)
    .bind(&data.message)
    .bind(&data.update_date)
    .bind(&data.date)
    .bind(&data.member_count)
    .bind(&data.delete_remaining_time)
    .bind(&data.notice_update_date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GuildInfo>> {
    sqlx::query_as::<_, GuildInfo>("SELECT * FROM GuildInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildInfo rows for a UID.
pub async fn delete_guild_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<GuildInfo> {
    sqlx::query_as::<_, GuildInfo>("SELECT * FROM GuildInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<GuildInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Look up the GuildInfo row for a guild by its public id, regardless of
/// which account's Uid it's stored under (the creator's, normally).
pub async fn get_by_guild_base_index(pool: &SqlitePool, guild_base_info_index: i64) -> sqlx::Result<Option<GuildInfo>> {
    sqlx::query_as::<_, GuildInfo>("SELECT * FROM GuildInfo WHERE GuildBaseInfoIndex = ? LIMIT 1")
        .bind(guild_base_info_index)
        .fetch_optional(pool)
        .await
}

/// Update the member count / notice timestamp on a guild's canonical
/// GuildInfo row (owned by whichever account created it).
pub async fn update_member_count(pool: &SqlitePool, guild_base_info_index: i64, delta: i32) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE GuildInfo SET MemberCount = COALESCE(MemberCount, 0) + ? WHERE GuildBaseInfoIndex = ?",
    )
    .bind(delta)
    .bind(guild_base_info_index)
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildInfo (
    Uid,
    GuildBaseInfoIndex,
    JoinType,
    Message,
    UpdateDate,
    Date,
    MemberCount,
    DeleteRemainingTime,
    NoticeUpdateDate
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
    .bind(&data.guild_base_info_index)
    .bind(&data.join_type)
    .bind(&data.message)
    .bind(&data.update_date)
    .bind(&data.date)
    .bind(&data.member_count)
    .bind(&data.delete_remaining_time)
    .bind(&data.notice_update_date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
