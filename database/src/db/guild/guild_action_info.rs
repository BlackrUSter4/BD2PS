use crate::models::game::guild::guild_action_info::GuildActionInfo;
use sqlx::SqlitePool;

/// Add a single GuildActionInfo record from a Rust struct.
pub async fn add_guild_action_info(pool: &SqlitePool, data: &GuildActionInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildActionInfo (
    Uid,
    Type,
    Time,
    GuildId,
    GuildName,
    IsNotify,
    Role
) VALUES (
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
    .bind(&data.r#type)
    .bind(&data.time)
    .bind(&data.guild_id)
    .bind(&data.guild_name)
    .bind(&data.is_notify)
    .bind(&data.role)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_action_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildActionInfo>> {
    sqlx::query_as::<_, GuildActionInfo>("SELECT * FROM GuildActionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildActionInfo rows for a UID.
pub async fn delete_guild_action_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildActionInfo WHERE Uid = ?")
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
) -> sqlx::Result<GuildActionInfo> {
    sqlx::query_as::<_, GuildActionInfo>(
        "SELECT * FROM GuildActionInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GuildActionInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildActionInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildActionInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildActionInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildActionInfo (
    Uid,
    Type,
    Time,
    GuildId,
    GuildName,
    IsNotify,
    Role
) VALUES (
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
    .bind(&data.r#type)
    .bind(&data.time)
    .bind(&data.guild_id)
    .bind(&data.guild_name)
    .bind(&data.is_notify)
    .bind(&data.role)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
