use crate::models::game::delayed::delayed_visibility_schedule_info::DelayedVisibilityScheduleInfo;
use sqlx::SqlitePool;

/// Add a single DelayedVisibilityScheduleInfo record from a Rust struct.
pub async fn add_delayed_visibility_schedule_info(
    pool: &SqlitePool,
    data: &DelayedVisibilityScheduleInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DelayedVisibilityScheduleInfo (
    Uid,
    Type,
    Id,
    TableId,
    OpenDate
) VALUES (
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
    .bind(&data.id)
    .bind(&data.table_id)
    .bind(&data.open_date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_delayed_visibility_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DelayedVisibilityScheduleInfo>> {
    sqlx::query_as::<_, DelayedVisibilityScheduleInfo>(
        "SELECT * FROM DelayedVisibilityScheduleInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all DelayedVisibilityScheduleInfo rows for a UID.
pub async fn delete_delayed_visibility_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DelayedVisibilityScheduleInfo WHERE Uid = ?")
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
) -> sqlx::Result<DelayedVisibilityScheduleInfo> {
    sqlx::query_as::<_, DelayedVisibilityScheduleInfo>(
        "SELECT * FROM DelayedVisibilityScheduleInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<DelayedVisibilityScheduleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DelayedVisibilityScheduleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DelayedVisibilityScheduleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DelayedVisibilityScheduleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DelayedVisibilityScheduleInfo (
    Uid,
    Type,
    Id,
    TableId,
    OpenDate
) VALUES (
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
    .bind(&data.id)
    .bind(&data.table_id)
    .bind(&data.open_date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
