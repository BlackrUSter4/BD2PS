use crate::models::game::mission::mission_info::MissionInfo;
use sqlx::SqlitePool;

/// Add a single MissionInfo record from a Rust struct.
pub async fn add_mission_info(pool: &SqlitePool, data: &MissionInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MissionInfo (
    Uid,
    GroupId,
    Id,
    GroupType,
    Value,
    IsComplete
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.group_type)
    .bind(&data.value)
    .bind(&data.is_complete)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mission_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MissionInfo>> {
    sqlx::query_as::<_, MissionInfo>("SELECT * FROM MissionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_group_and_id(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    id: i32,
) -> sqlx::Result<Option<MissionInfo>> {
    sqlx::query_as::<_, MissionInfo>("SELECT * FROM MissionInfo WHERE Uid = ? AND GroupId = ? AND Id = ?")
        .bind(uid)
        .bind(group_id)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_progress(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    id: i32,
    group_type: Option<i32>,
    value: i32,
    is_complete: bool,
) -> sqlx::Result<()> {
    if get_by_group_and_id(pool, uid, group_id, id).await?.is_some() {
        sqlx::query("UPDATE MissionInfo SET Value = ?, IsComplete = ? WHERE Uid = ? AND GroupId = ? AND Id = ?")
            .bind(value)
            .bind(is_complete)
            .bind(uid)
            .bind(group_id)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        add_mission_info(
            pool,
            &MissionInfo { index: 0, uid, group_id: Some(group_id), id: Some(id), group_type, value: Some(value), is_complete: Some(is_complete) },
        )
        .await?;
    }
    Ok(())
}

pub async fn get_completed(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MissionInfo>> {
    sqlx::query_as::<_, MissionInfo>("SELECT * FROM MissionInfo WHERE Uid = ? AND IsComplete = 1")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn delete_by_group_and_id(pool: &SqlitePool, uid: i64, group_id: i32, id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MissionInfo WHERE Uid = ? AND GroupId = ? AND Id = ?")
        .bind(uid)
        .bind(group_id)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all MissionInfo rows for a UID.
pub async fn delete_mission_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MissionInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MissionInfo> {
    sqlx::query_as::<_, MissionInfo>("SELECT * FROM MissionInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MissionInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MissionInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MissionInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MissionInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MissionInfo (
    Uid,
    GroupId,
    Id,
    GroupType,
    Value,
    IsComplete
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.group_type)
    .bind(&data.value)
    .bind(&data.is_complete)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
