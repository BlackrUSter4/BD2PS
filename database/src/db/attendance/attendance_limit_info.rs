use crate::models::game::attendance::attendance_limit_info::AttendanceLimitInfo;
use sqlx::SqlitePool;

/// Add a single AttendanceLimitInfo record from a Rust struct.
pub async fn add_attendance_limit_info(
    pool: &SqlitePool,
    data: &AttendanceLimitInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO AttendanceLimitInfo (
    Uid,
    Id,
    RewardId,
    RewardDate
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
    .bind(&data.reward_id)
    .bind(&data.reward_date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_attendance_limit_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<AttendanceLimitInfo>> {
    sqlx::query_as::<_, AttendanceLimitInfo>("SELECT * FROM AttendanceLimitInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all AttendanceLimitInfo rows for a UID.
pub async fn delete_attendance_limit_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM AttendanceLimitInfo WHERE Uid = ?")
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
) -> sqlx::Result<AttendanceLimitInfo> {
    sqlx::query_as::<_, AttendanceLimitInfo>(
        "SELECT * FROM AttendanceLimitInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<AttendanceLimitInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM AttendanceLimitInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, AttendanceLimitInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &AttendanceLimitInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO AttendanceLimitInfo (
    Uid,
    Id,
    RewardId,
    RewardDate
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
    .bind(&data.reward_id)
    .bind(&data.reward_date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
