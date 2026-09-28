use crate::models::game::sky::sky_way_schedule_info::SkyWayScheduleInfo;
use sqlx::SqlitePool;

/// Add a single SkyWayScheduleInfo record from a Rust struct.
pub async fn add_sky_way_schedule_info(
    pool: &SqlitePool,
    data: &SkyWayScheduleInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SkyWayScheduleInfo (
    Uid,
    GroupId,
    BonusRate
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.bonus_rate)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_sky_way_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<SkyWayScheduleInfo>> {
    sqlx::query_as::<_, SkyWayScheduleInfo>("SELECT * FROM SkyWayScheduleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all SkyWayScheduleInfo rows for a UID.
pub async fn delete_sky_way_schedule_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SkyWayScheduleInfo WHERE Uid = ?")
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
) -> sqlx::Result<SkyWayScheduleInfo> {
    sqlx::query_as::<_, SkyWayScheduleInfo>(
        "SELECT * FROM SkyWayScheduleInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<SkyWayScheduleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM SkyWayScheduleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, SkyWayScheduleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &SkyWayScheduleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO SkyWayScheduleInfo (
    Uid,
    GroupId,
    BonusRate
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.bonus_rate)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
