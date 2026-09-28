use crate::models::game::schedule::schedule_info::ScheduleInfo;
use sqlx::SqlitePool;

/// Add a single ScheduleInfo record from a Rust struct.
pub async fn add_schedule_info(pool: &SqlitePool, data: &ScheduleInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ScheduleInfo (
    Uid,
    ContentId,
    CurrentSeason,
    NextSeason
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.content_id)
    .bind(&data.current_season)
    .bind(&data.next_season)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_schedule_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ScheduleInfo>> {
    sqlx::query_as::<_, ScheduleInfo>("SELECT * FROM ScheduleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ScheduleInfo rows for a UID.
pub async fn delete_schedule_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ScheduleInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ScheduleInfo> {
    sqlx::query_as::<_, ScheduleInfo>("SELECT * FROM ScheduleInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ScheduleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ScheduleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ScheduleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ScheduleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ScheduleInfo (
    Uid,
    ContentId,
    CurrentSeason,
    NextSeason
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.content_id)
    .bind(&data.current_season)
    .bind(&data.next_season)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
