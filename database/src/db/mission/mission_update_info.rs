use crate::models::game::mission::mission_update_info::MissionUpdateInfo;
use sqlx::SqlitePool;

/// Add a single MissionUpdateInfo record from a Rust struct.
pub async fn add_mission_update_info(
    pool: &SqlitePool,
    data: &MissionUpdateInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MissionUpdateInfo (
    Uid,
    GroupId,
    Id,
    Value,
    EventId
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.value)
    .bind(&data.event_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mission_update_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MissionUpdateInfo>> {
    sqlx::query_as::<_, MissionUpdateInfo>("SELECT * FROM MissionUpdateInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MissionUpdateInfo rows for a UID.
pub async fn delete_mission_update_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MissionUpdateInfo WHERE Uid = ?")
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
) -> sqlx::Result<MissionUpdateInfo> {
    sqlx::query_as::<_, MissionUpdateInfo>(
        "SELECT * FROM MissionUpdateInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MissionUpdateInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MissionUpdateInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MissionUpdateInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MissionUpdateInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MissionUpdateInfo (
    Uid,
    GroupId,
    Id,
    Value,
    EventId
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.value)
    .bind(&data.event_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
