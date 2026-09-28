use crate::models::game::evil::evil_castle_rogue_like_growth_info::EvilCastleRogueLikeGrowthInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeGrowthInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_growth_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeGrowthInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeGrowthInfo (
    Uid,
    Type,
    Level
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.level)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_growth_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeGrowthInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeGrowthInfo>(
        "SELECT * FROM EvilCastleRogueLikeGrowthInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeGrowthInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_growth_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeGrowthInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeGrowthInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeGrowthInfo>(
        "SELECT * FROM EvilCastleRogueLikeGrowthInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeGrowthInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeGrowthInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeGrowthInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleRogueLikeGrowthInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeGrowthInfo (
    Uid,
    Type,
    Level
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.level)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Set (insert-or-replace) the level for one growth type.
pub async fn set_level(pool: &SqlitePool, uid: i64, r#type: i32, level: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeGrowthInfo WHERE Uid = ? AND Type = ?")
        .bind(uid)
        .bind(r#type)
        .execute(pool)
        .await?;
    sqlx::query("INSERT INTO EvilCastleRogueLikeGrowthInfo (Uid, Type, Level) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(r#type)
        .bind(level)
        .execute(pool)
        .await?;
    Ok(())
}
