use crate::models::game::guild::guild_base_info::GuildBaseInfo;
use sqlx::SqlitePool;

/// Add a single GuildBaseInfo record from a Rust struct.
pub async fn add_guild_base_info(pool: &SqlitePool, data: &GuildBaseInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildBaseInfo (
    Uid,
    Id,
    Name,
    Icon,
    IconColor,
    Grade
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
    .bind(&data.id)
    .bind(&data.name)
    .bind(&data.icon)
    .bind(&data.icon_color)
    .bind(&data.grade)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_base_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GuildBaseInfo>> {
    sqlx::query_as::<_, GuildBaseInfo>("SELECT * FROM GuildBaseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildBaseInfo rows for a UID.
pub async fn delete_guild_base_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildBaseInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<GuildBaseInfo> {
    sqlx::query_as::<_, GuildBaseInfo>("SELECT * FROM GuildBaseInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<GuildBaseInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildBaseInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildBaseInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Cross-account lookup by the guild's public Id (guilds live under their
/// creator's Uid; other accounts need to find them regardless of owner).
pub async fn get_by_guild_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<GuildBaseInfo>> {
    sqlx::query_as::<_, GuildBaseInfo>("SELECT * FROM GuildBaseInfo WHERE Id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Cross-account name search (case-insensitive substring), for GuildSearch.
pub async fn search_by_name(pool: &SqlitePool, name: &str, limit: i64) -> sqlx::Result<Vec<GuildBaseInfo>> {
    sqlx::query_as::<_, GuildBaseInfo>(
        "SELECT * FROM GuildBaseInfo WHERE Name LIKE '%' || ? || '%' ESCAPE '\\' LIMIT ?",
    )
    .bind(name)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// All guilds across every account (for GuildRecommend), newest first.
pub async fn get_all_guilds(pool: &SqlitePool, limit: i64) -> sqlx::Result<Vec<GuildBaseInfo>> {
    sqlx::query_as::<_, GuildBaseInfo>("SELECT * FROM GuildBaseInfo ORDER BY \"Index\" DESC LIMIT ?")
        .bind(limit)
        .fetch_all(pool)
        .await
}

/// Update mutable guild fields (name/icon/message handled via GuildInfo) by public Id.
pub async fn update_by_guild_id(
    pool: &SqlitePool,
    id: i64,
    name: Option<&str>,
    icon: Option<i32>,
    icon_color: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE GuildBaseInfo SET Name = COALESCE(?, Name), Icon = COALESCE(?, Icon), IconColor = COALESCE(?, IconColor) WHERE Id = ?",
    )
    .bind(name)
    .bind(icon)
    .bind(icon_color)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildBaseInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildBaseInfo (
    Uid,
    Id,
    Name,
    Icon,
    IconColor,
    Grade
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
    .bind(&data.id)
    .bind(&data.name)
    .bind(&data.icon)
    .bind(&data.icon_color)
    .bind(&data.grade)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
