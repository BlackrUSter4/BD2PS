use crate::models::game::subscribe::subscribe_attendance_info::SubscribeAttendanceInfo;
use sqlx::SqlitePool;

/// Add a single SubscribeAttendanceInfo record from a Rust struct.
pub async fn add_subscribe_attendance_info(
    pool: &SqlitePool,
    data: &SubscribeAttendanceInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SubscribeAttendanceInfo (
    Uid,
    TicketId,
    ReservedDate,
    ExpiryDate
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.ticket_id)
    .bind(&data.reserved_date)
    .bind(&data.expiry_date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_subscribe_attendance_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<SubscribeAttendanceInfo>> {
    sqlx::query_as::<_, SubscribeAttendanceInfo>(
        "SELECT * FROM SubscribeAttendanceInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all SubscribeAttendanceInfo rows for a UID.
pub async fn delete_subscribe_attendance_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SubscribeAttendanceInfo WHERE Uid = ?")
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
) -> sqlx::Result<SubscribeAttendanceInfo> {
    sqlx::query_as::<_, SubscribeAttendanceInfo>(
        "SELECT * FROM SubscribeAttendanceInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<SubscribeAttendanceInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM SubscribeAttendanceInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, SubscribeAttendanceInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &SubscribeAttendanceInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO SubscribeAttendanceInfo (
    Uid,
    TicketId,
    ReservedDate,
    ExpiryDate
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.ticket_id)
    .bind(&data.reserved_date)
    .bind(&data.expiry_date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
