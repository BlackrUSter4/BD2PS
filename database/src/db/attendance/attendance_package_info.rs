use crate::models::game::attendance::attendance_package_info::AttendancePackageInfo;
use sqlx::SqlitePool;

/// Add a single AttendancePackageInfo record from a Rust struct.
pub async fn add_attendance_package_info(
    pool: &SqlitePool,
    data: &AttendancePackageInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO AttendancePackageInfo (
    Uid,
    GroupId,
    Id,
    IsReward
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.is_reward)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_attendance_package_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<AttendancePackageInfo>> {
    sqlx::query_as::<_, AttendancePackageInfo>("SELECT * FROM AttendancePackageInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all AttendancePackageInfo rows for a UID.
pub async fn delete_attendance_package_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM AttendancePackageInfo WHERE Uid = ?")
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
) -> sqlx::Result<AttendancePackageInfo> {
    sqlx::query_as::<_, AttendancePackageInfo>(
        "SELECT * FROM AttendancePackageInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<AttendancePackageInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM AttendancePackageInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, AttendancePackageInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &AttendancePackageInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO AttendancePackageInfo (
    Uid,
    GroupId,
    Id,
    IsReward
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.is_reward)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
