use crate::models::game::evil::evil_castle_rogue_like_floor_info::EvilCastleRogueLikeFloorInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeFloorInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_floor_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeFloorInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeFloorInfo (
    Uid,
    Number
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.number)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_floor_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeFloorInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeFloorInfo>(
        "SELECT * FROM EvilCastleRogueLikeFloorInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeFloorInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_floor_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeFloorInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeFloorInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeFloorInfo>(
        "SELECT * FROM EvilCastleRogueLikeFloorInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeFloorInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeFloorInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeFloorInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleRogueLikeFloorInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeFloorInfo (
    Uid,
    Number
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.number)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Insert a floor-visited marker if it isn't already recorded.
pub async fn mark_visited(pool: &SqlitePool, uid: i64, number: i32) -> sqlx::Result<()> {
    let existing: Option<(i64,)> = sqlx::query_as(
        "SELECT Index FROM EvilCastleRogueLikeFloorInfo WHERE Uid = ? AND Number = ?",
    )
    .bind(uid)
    .bind(number)
    .fetch_optional(pool)
    .await?;
    if existing.is_none() {
        sqlx::query("INSERT INTO EvilCastleRogueLikeFloorInfo (Uid, Number) VALUES (?, ?)")
            .bind(uid)
            .bind(number)
            .execute(pool)
            .await?;
    }
    Ok(())
}
