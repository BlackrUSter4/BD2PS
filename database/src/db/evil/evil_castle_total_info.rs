use crate::models::game::evil::evil_castle_total_info::EvilCastleTotalInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleTotalInfo record from a Rust struct.
pub async fn add_evil_castle_total_info(
    pool: &SqlitePool,
    data: &EvilCastleTotalInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleTotalInfo (
    Uid,
    Rank,
    Point,
    IsRewarded
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.point)
    .bind(&data.is_rewarded)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_total_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleTotalInfo>> {
    sqlx::query_as::<_, EvilCastleTotalInfo>("SELECT * FROM EvilCastleTotalInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EvilCastleTotalInfo rows for a UID.
pub async fn delete_evil_castle_total_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleTotalInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleTotalInfo> {
    sqlx::query_as::<_, EvilCastleTotalInfo>(
        "SELECT * FROM EvilCastleTotalInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleTotalInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleTotalInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleTotalInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleTotalInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleTotalInfo (
    Uid,
    Rank,
    Point,
    IsRewarded
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.point)
    .bind(&data.is_rewarded)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// One live row per uid — replace it (delete-then-insert).
pub async fn upsert(pool: &SqlitePool, data: &EvilCastleTotalInfo) -> sqlx::Result<()> {
    delete_evil_castle_total_info(pool, data.uid).await?;
    insert(pool, data).await.map(|_| ())
}
