use crate::models::game::pack::pack_event_story_info::PackEventStoryInfo;
use sqlx::SqlitePool;

/// Add a single PackEventStoryInfo record from a Rust struct.
pub async fn add_pack_event_story_info(
    pool: &SqlitePool,
    data: &PackEventStoryInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackEventStoryInfo (
    Uid,
    EventUid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_event_story_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PackEventStoryInfo>> {
    sqlx::query_as::<_, PackEventStoryInfo>("SELECT * FROM PackEventStoryInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Whether a (group_id, id) story has already been cleared for this account.
pub async fn is_cleared(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    id: i32,
) -> sqlx::Result<bool> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM PackEventStoryInfo WHERE Uid = ? AND GroupId = ? AND Id = ?",
    )
    .bind(uid)
    .bind(group_id)
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(row.0 > 0)
}

/// Mark a (event_uid, group_id, id) story cleared, if not already recorded.
pub async fn mark_cleared(
    pool: &SqlitePool,
    uid: i64,
    event_uid: i32,
    group_id: i32,
    id: i32,
) -> sqlx::Result<()> {
    if is_cleared(pool, uid, group_id, id).await? {
        return Ok(());
    }
    sqlx::query("INSERT INTO PackEventStoryInfo (Uid, EventUid, GroupId, Id) VALUES (?, ?, ?, ?)")
        .bind(uid)
        .bind(event_uid)
        .bind(group_id)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all PackEventStoryInfo rows for a UID.
pub async fn delete_pack_event_story_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackEventStoryInfo WHERE Uid = ?")
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
) -> sqlx::Result<PackEventStoryInfo> {
    sqlx::query_as::<_, PackEventStoryInfo>(
        "SELECT * FROM PackEventStoryInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PackEventStoryInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PackEventStoryInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PackEventStoryInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PackEventStoryInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PackEventStoryInfo (
    Uid,
    EventUid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
