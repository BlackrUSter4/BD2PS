use crate::models::game::event::event_exchange_reward_info::EventExchangeRewardInfo;
use sqlx::SqlitePool;

/// Add a single EventExchangeRewardInfo record from a Rust struct.
pub async fn add_event_exchange_reward_info(
    pool: &SqlitePool,
    data: &EventExchangeRewardInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EventExchangeRewardInfo (
    Uid,
    EventUid,
    GroupId,
    Id,
    Count
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
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_event_exchange_reward_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EventExchangeRewardInfo>> {
    sqlx::query_as::<_, EventExchangeRewardInfo>(
        "SELECT * FROM EventExchangeRewardInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Get the row for one (event_uid, group_id, id), if any exchanges have happened.
pub async fn get_by_uid_event_group_id(
    pool: &SqlitePool,
    uid: i64,
    event_uid: i32,
    group_id: i32,
    id: i32,
) -> sqlx::Result<Option<EventExchangeRewardInfo>> {
    sqlx::query_as::<_, EventExchangeRewardInfo>(
        "SELECT * FROM EventExchangeRewardInfo WHERE Uid = ? AND EventUid = ? AND GroupId = ? AND Id = ?",
    )
    .bind(uid)
    .bind(event_uid)
    .bind(group_id)
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Add to the exchanged count for one (event_uid, group_id, id).
pub async fn add_count(pool: &SqlitePool, uid: i64, event_uid: i32, group_id: i32, id: i32, delta: i32) -> sqlx::Result<()> {
    if get_by_uid_event_group_id(pool, uid, event_uid, group_id, id).await?.is_some() {
        sqlx::query("UPDATE EventExchangeRewardInfo SET Count = COALESCE(Count, 0) + ? WHERE Uid = ? AND EventUid = ? AND GroupId = ? AND Id = ?")
            .bind(delta)
            .bind(uid)
            .bind(event_uid)
            .bind(group_id)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        add_event_exchange_reward_info(
            pool,
            &EventExchangeRewardInfo { index: 0, uid, event_uid: Some(event_uid), group_id: Some(group_id), id: Some(id), count: Some(delta) },
        ).await?;
    }
    Ok(())
}

/// Delete all EventExchangeRewardInfo rows for a UID.
pub async fn delete_event_exchange_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EventExchangeRewardInfo WHERE Uid = ?")
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
) -> sqlx::Result<EventExchangeRewardInfo> {
    sqlx::query_as::<_, EventExchangeRewardInfo>(
        "SELECT * FROM EventExchangeRewardInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EventExchangeRewardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EventExchangeRewardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EventExchangeRewardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EventExchangeRewardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EventExchangeRewardInfo (
    Uid,
    EventUid,
    GroupId,
    Id,
    Count
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
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
