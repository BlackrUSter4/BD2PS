use crate::models::game::event::event_schedule_info::EventScheduleInfo;
use sqlx::SqlitePool;

/// Add a single EventScheduleInfo record from a Rust struct.
pub async fn add_event_schedule_info(
    pool: &SqlitePool,
    data: &EventScheduleInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EventScheduleInfo (
    Uid,
    Id,
    EventType,
    EventId,
    EventSubId,
    StartDate,
    EndDate,
    IsActive
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
    .bind(&data.id)
    .bind(&data.event_type)
    .bind(&data.event_id)
    .bind(&data.event_sub_id)
    .bind(&data.start_date)
    .bind(&data.end_date)
    .bind(&data.is_active)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_event_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EventScheduleInfo>> {
    sqlx::query_as::<_, EventScheduleInfo>("SELECT * FROM EventScheduleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EventScheduleInfo rows for a UID.
pub async fn delete_event_schedule_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EventScheduleInfo WHERE Uid = ?")
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
) -> sqlx::Result<EventScheduleInfo> {
    sqlx::query_as::<_, EventScheduleInfo>(
        "SELECT * FROM EventScheduleInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EventScheduleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EventScheduleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EventScheduleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EventScheduleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EventScheduleInfo (
    Uid,
    Id,
    EventType,
    EventId,
    EventSubId,
    StartDate,
    EndDate,
    IsActive
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
    .bind(&data.id)
    .bind(&data.event_type)
    .bind(&data.event_id)
    .bind(&data.event_sub_id)
    .bind(&data.start_date)
    .bind(&data.end_date)
    .bind(&data.is_active)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
