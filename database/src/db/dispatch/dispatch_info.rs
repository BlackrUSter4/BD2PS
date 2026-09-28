use crate::models::game::dispatch::dispatch_info::DispatchInfo;
use sqlx::SqlitePool;

/// Add a single DispatchInfo record from a Rust struct.
pub async fn add_dispatch_info(pool: &SqlitePool, data: &DispatchInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DispatchInfo (
    Uid,
    Id,
    ServerNowTime,
    EndTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.server_now_time)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_dispatch_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<DispatchInfo>> {
    sqlx::query_as::<_, DispatchInfo>("SELECT * FROM DispatchInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_id(pool: &SqlitePool, uid: i64, id: i32) -> sqlx::Result<Option<DispatchInfo>> {
    sqlx::query_as::<_, DispatchInfo>("SELECT * FROM DispatchInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_by_id(pool: &SqlitePool, uid: i64, id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DispatchInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all DispatchInfo rows for a UID.
pub async fn delete_dispatch_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DispatchInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<DispatchInfo> {
    sqlx::query_as::<_, DispatchInfo>("SELECT * FROM DispatchInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<DispatchInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DispatchInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DispatchInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DispatchInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DispatchInfo (
    Uid,
    Id,
    ServerNowTime,
    EndTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.server_now_time)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
