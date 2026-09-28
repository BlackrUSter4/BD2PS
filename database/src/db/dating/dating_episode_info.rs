use crate::models::game::dating::dating_episode_info::DatingEpisodeInfo;
use sqlx::SqlitePool;

/// Add a single DatingEpisodeInfo record from a Rust struct.
pub async fn add_dating_episode_info(
    pool: &SqlitePool,
    data: &DatingEpisodeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DatingEpisodeInfo (
    Uid,
    GroupId,
    DatingPoint,
    LastClearId,
    LastMessageGroupId,
    LastMessageId,
    LastMessageUpdateTime
) VALUES (
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
    .bind(&data.group_id)
    .bind(&data.dating_point)
    .bind(&data.last_clear_id)
    .bind(&data.last_message_group_id)
    .bind(&data.last_message_id)
    .bind(&data.last_message_update_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_dating_episode_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DatingEpisodeInfo>> {
    sqlx::query_as::<_, DatingEpisodeInfo>("SELECT * FROM DatingEpisodeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all DatingEpisodeInfo rows for a UID.
pub async fn get_by_group(pool: &SqlitePool, uid: i64, group_id: i32) -> sqlx::Result<Option<DatingEpisodeInfo>> {
    sqlx::query_as::<_, DatingEpisodeInfo>("SELECT * FROM DatingEpisodeInfo WHERE Uid = ? AND GroupId = ?")
        .bind(uid)
        .bind(group_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_or_create(pool: &SqlitePool, uid: i64, group_id: i32) -> sqlx::Result<DatingEpisodeInfo> {
    if let Some(row) = get_by_group(pool, uid, group_id).await? {
        return Ok(row);
    }
    add_dating_episode_info(
        pool,
        &DatingEpisodeInfo { index: 0, uid, group_id: Some(group_id), dating_point: Some(0), last_clear_id: Some(0), last_message_group_id: None, last_message_id: None, last_message_update_time: None },
    )
    .await?;
    Ok(get_by_group(pool, uid, group_id).await?.expect("row was just inserted"))
}

pub async fn set_last_clear_id(pool: &SqlitePool, uid: i64, group_id: i32, last_clear_id: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE DatingEpisodeInfo SET LastClearId = ? WHERE Uid = ? AND GroupId = ?")
        .bind(last_clear_id)
        .bind(uid)
        .bind(group_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_last_message(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    last_message_group_id: i32,
    last_message_id: i32,
    last_message_update_time: i64,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE DatingEpisodeInfo SET LastMessageGroupId = ?, LastMessageId = ?, LastMessageUpdateTime = ? WHERE Uid = ? AND GroupId = ?",
    )
    .bind(last_message_group_id)
    .bind(last_message_id)
    .bind(last_message_update_time)
    .bind(uid)
    .bind(group_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_dating_episode_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DatingEpisodeInfo WHERE Uid = ?")
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
) -> sqlx::Result<DatingEpisodeInfo> {
    sqlx::query_as::<_, DatingEpisodeInfo>(
        "SELECT * FROM DatingEpisodeInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<DatingEpisodeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DatingEpisodeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DatingEpisodeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DatingEpisodeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DatingEpisodeInfo (
    Uid,
    GroupId,
    DatingPoint,
    LastClearId,
    LastMessageGroupId,
    LastMessageId,
    LastMessageUpdateTime
) VALUES (
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
    .bind(&data.group_id)
    .bind(&data.dating_point)
    .bind(&data.last_clear_id)
    .bind(&data.last_message_group_id)
    .bind(&data.last_message_id)
    .bind(&data.last_message_update_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
