use crate::models::game::hunt::hunt_dispatch_info::HuntDispatchInfo;
use sqlx::SqlitePool;

/// Add a single HuntDispatchInfo record from a Rust struct.
pub async fn add_hunt_dispatch_info(
    pool: &SqlitePool,
    data: &HuntDispatchInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO HuntDispatchInfo (
    Uid,
    HuntDispatchGroupId,
    HuntDispatchId,
    Count,
    StartTime,
    EndTime,
    DecreaseFreeApCount,
    DecreaseCashApCount
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
    .bind(&data.hunt_dispatch_group_id)
    .bind(&data.hunt_dispatch_id)
    .bind(&data.count)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.decrease_free_ap_count)
    .bind(&data.decrease_cash_ap_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_hunt_dispatch_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<HuntDispatchInfo>> {
    sqlx::query_as::<_, HuntDispatchInfo>("SELECT * FROM HuntDispatchInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all HuntDispatchInfo rows for a UID.
pub async fn delete_hunt_dispatch_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM HuntDispatchInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_by_group_and_id(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    id: i32,
) -> sqlx::Result<Option<HuntDispatchInfo>> {
    sqlx::query_as::<_, HuntDispatchInfo>(
        "SELECT * FROM HuntDispatchInfo WHERE Uid = ? AND HuntDispatchGroupId = ? AND HuntDispatchId = ?",
    )
    .bind(uid)
    .bind(group_id)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM HuntDispatchInfo WHERE Uid = ? AND \"Index\" = ?")
        .bind(uid)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<HuntDispatchInfo> {
    sqlx::query_as::<_, HuntDispatchInfo>(
        "SELECT * FROM HuntDispatchInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<HuntDispatchInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM HuntDispatchInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, HuntDispatchInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &HuntDispatchInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO HuntDispatchInfo (
    Uid,
    HuntDispatchGroupId,
    HuntDispatchId,
    Count,
    StartTime,
    EndTime,
    DecreaseFreeApCount,
    DecreaseCashApCount
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
    .bind(&data.hunt_dispatch_group_id)
    .bind(&data.hunt_dispatch_id)
    .bind(&data.count)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.decrease_free_ap_count)
    .bind(&data.decrease_cash_ap_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
