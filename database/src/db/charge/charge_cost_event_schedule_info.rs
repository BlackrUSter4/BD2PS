use crate::models::game::charge::charge_cost_event_schedule_info::ChargeCostEventScheduleInfo;
use sqlx::SqlitePool;

/// Add a single ChargeCostEventScheduleInfo record from a Rust struct.
pub async fn add_charge_cost_event_schedule_info(
    pool: &SqlitePool,
    data: &ChargeCostEventScheduleInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ChargeCostEventScheduleInfo (
    Uid,
    ScheduleIndex,
    ItemType,
    MaxCount,
    StartTime,
    EndTime
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
    .bind(&data.schedule_index)
    .bind(&data.item_type)
    .bind(&data.max_count)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_charge_cost_event_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ChargeCostEventScheduleInfo>> {
    sqlx::query_as::<_, ChargeCostEventScheduleInfo>(
        "SELECT * FROM ChargeCostEventScheduleInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all ChargeCostEventScheduleInfo rows for a UID.
pub async fn delete_charge_cost_event_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ChargeCostEventScheduleInfo WHERE Uid = ?")
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
) -> sqlx::Result<ChargeCostEventScheduleInfo> {
    sqlx::query_as::<_, ChargeCostEventScheduleInfo>(
        "SELECT * FROM ChargeCostEventScheduleInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<ChargeCostEventScheduleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ChargeCostEventScheduleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ChargeCostEventScheduleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ChargeCostEventScheduleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ChargeCostEventScheduleInfo (
    Uid,
    ScheduleIndex,
    ItemType,
    MaxCount,
    StartTime,
    EndTime
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
    .bind(&data.schedule_index)
    .bind(&data.item_type)
    .bind(&data.max_count)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
