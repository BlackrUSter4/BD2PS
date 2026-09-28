use crate::models::game::sky::sky_way_info::SkyWayInfo;
use sqlx::SqlitePool;

/// Add a single SkyWayInfo record from a Rust struct.
pub async fn add_sky_way_info(pool: &SqlitePool, data: &SkyWayInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SkyWayInfo (
    Uid,
    IsAuto,
    GroupId,
    CurrentId,
    MaxClearLevel
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
    .bind(&data.is_auto)
    .bind(&data.group_id)
    .bind(&data.current_id)
    .bind(&data.max_clear_level)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_sky_way_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<SkyWayInfo>> {
    sqlx::query_as::<_, SkyWayInfo>("SELECT * FROM SkyWayInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all SkyWayInfo rows for a UID.
pub async fn get_by_group(pool: &SqlitePool, uid: i64, group_id: i32) -> sqlx::Result<Option<SkyWayInfo>> {
    sqlx::query_as::<_, SkyWayInfo>("SELECT * FROM SkyWayInfo WHERE Uid = ? AND GroupId = ?")
        .bind(uid)
        .bind(group_id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_progress(pool: &SqlitePool, uid: i64, group_id: i32, current_id: i32, is_auto: Option<bool>) -> sqlx::Result<()> {
    if let Some(row) = get_by_group(pool, uid, group_id).await? {
        let max_clear_level = row.max_clear_level.unwrap_or(0).max(current_id);
        sqlx::query("UPDATE SkyWayInfo SET CurrentId = ?, MaxClearLevel = ?, IsAuto = ? WHERE \"Index\" = ?")
            .bind(current_id)
            .bind(max_clear_level)
            .bind(is_auto)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_sky_way_info(
            pool,
            &SkyWayInfo { index: 0, uid, is_auto, group_id: Some(group_id), current_id: Some(current_id), max_clear_level: Some(current_id) },
        )
        .await?;
    }
    Ok(())
}

pub async fn delete_sky_way_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SkyWayInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<SkyWayInfo> {
    sqlx::query_as::<_, SkyWayInfo>("SELECT * FROM SkyWayInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<SkyWayInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM SkyWayInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, SkyWayInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &SkyWayInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO SkyWayInfo (
    Uid,
    IsAuto,
    GroupId,
    CurrentId,
    MaxClearLevel
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
    .bind(&data.is_auto)
    .bind(&data.group_id)
    .bind(&data.current_id)
    .bind(&data.max_clear_level)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
