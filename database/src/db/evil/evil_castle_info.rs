use crate::models::game::evil::evil_castle_info::EvilCastleInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleInfo record from a Rust struct.
pub async fn add_evil_castle_info(pool: &SqlitePool, data: &EvilCastleInfo) -> sqlx::Result<()> {
    insert(pool, data).await.map(|_| ())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleInfo>> {
    sqlx::query_as::<_, EvilCastleInfo>("SELECT * FROM EvilCastleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Fetch the single row for a given UID + tower (pack_id), if any.
pub async fn get_by_pack_id(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
) -> sqlx::Result<Option<EvilCastleInfo>> {
    sqlx::query_as::<_, EvilCastleInfo>(
        "SELECT * FROM EvilCastleInfo WHERE Uid = ? AND PackId = ?",
    )
    .bind(uid)
    .bind(pack_id)
    .fetch_optional(pool)
    .await
}

/// Delete all EvilCastleInfo rows for a UID.
pub async fn delete_evil_castle_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<EvilCastleInfo> {
    sqlx::query_as::<_, EvilCastleInfo>("SELECT * FROM EvilCastleInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<EvilCastleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleInfo (
    Uid,
    PackId,
    Rank,
    StageIndex,
    Retry,
    Point,
    SeasonHighestPoint,
    IsRewarded,
    StageClearTime
) VALUES (
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.rank)
    .bind(&data.stage_index)
    .bind(&data.retry)
    .bind(&data.point)
    .bind(&data.season_highest_point)
    .bind(&data.is_rewarded)
    .bind(&data.stage_clear_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Top accounts by point for a given tower, real cross-account aggregation.
pub async fn top_by_pack_id(
    pool: &SqlitePool,
    pack_id: i32,
    limit: i64,
) -> sqlx::Result<Vec<EvilCastleInfo>> {
    sqlx::query_as::<_, EvilCastleInfo>(
        "SELECT * FROM EvilCastleInfo WHERE PackId = ? ORDER BY Point DESC LIMIT ?",
    )
    .bind(pack_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// This account's rank among everyone in a given tower (1-based).
pub async fn rank_for(pool: &SqlitePool, uid: i64, pack_id: i32) -> sqlx::Result<i64> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) + 1 FROM EvilCastleInfo WHERE PackId = ? AND Point > (
            SELECT COALESCE(MAX(Point), 0) FROM EvilCastleInfo WHERE Uid = ? AND PackId = ?
        )",
    )
    .bind(pack_id)
    .bind(uid)
    .bind(pack_id)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Replace the row for a given UID + pack_id (delete-then-insert, matching
/// this codebase's established convention for "update" since no UPDATE
/// helper exists anywhere in this db layer).
pub async fn upsert(pool: &SqlitePool, data: &EvilCastleInfo) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleInfo WHERE Uid = ? AND PackId = ?")
        .bind(&data.uid)
        .bind(&data.pack_id)
        .execute(pool)
        .await?;
    insert(pool, data).await.map(|_| ())
}
