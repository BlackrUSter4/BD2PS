use crate::models::game::event::event_reward_history_info::EventRewardHistoryInfo;
use sqlx::SqlitePool;

/// Add a single EventRewardHistoryInfo record from a Rust struct.
pub async fn add_event_reward_history_info(
    pool: &SqlitePool,
    data: &EventRewardHistoryInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EventRewardHistoryInfo (
    Uid,
    EventScheduleId,
    EventGroupId,
    RewardId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(data.uid)
    .bind(data.event_schedule_id)
    .bind(data.event_group_id)
    .bind(data.reward_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_event_reward_history_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EventRewardHistoryInfo>> {
    sqlx::query_as::<_, EventRewardHistoryInfo>(
        "SELECT * FROM EventRewardHistoryInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Whether a specific reward has already been claimed by this account.
pub async fn has_claimed(
    pool: &SqlitePool,
    uid: i64,
    event_schedule_id: i32,
    event_group_id: i32,
    reward_id: i32,
) -> sqlx::Result<bool> {
    let row = sqlx::query_as::<_, EventRewardHistoryInfo>(
        "SELECT * FROM EventRewardHistoryInfo WHERE Uid = ? AND EventScheduleId = ? AND EventGroupId = ? AND RewardId = ?",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(event_group_id)
    .bind(reward_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.is_some())
}

/// Record a claim.
pub async fn mark_claimed(
    pool: &SqlitePool,
    uid: i64,
    event_schedule_id: i32,
    event_group_id: i32,
    reward_id: i32,
) -> sqlx::Result<()> {
    add_event_reward_history_info(
        pool,
        &EventRewardHistoryInfo {
            index: 0,
            uid,
            event_schedule_id: Some(event_schedule_id),
            event_group_id: Some(event_group_id),
            reward_id,
        },
    )
    .await
}

/// Delete all EventRewardHistoryInfo rows for a UID.
pub async fn delete_event_reward_history_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EventRewardHistoryInfo WHERE Uid = ?")
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
) -> sqlx::Result<EventRewardHistoryInfo> {
    sqlx::query_as::<_, EventRewardHistoryInfo>(
        "SELECT * FROM EventRewardHistoryInfo WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EventRewardHistoryInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EventRewardHistoryInfo (
    Uid,
    EventScheduleId,
    EventGroupId,
    RewardId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(data.uid)
    .bind(data.event_schedule_id)
    .bind(data.event_group_id)
    .bind(data.reward_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
