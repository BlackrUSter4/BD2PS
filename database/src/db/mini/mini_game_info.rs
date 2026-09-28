use crate::models::game::mini::mini_game_info::MiniGameInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameInfo record from a Rust struct.
pub async fn add_mini_game_info(pool: &SqlitePool, data: &MiniGameInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameInfo (
    Uid,
    EventScheduleId,
    LastRewardPoint,
    BestRecordValue,
    IsPossibleQuickReward,
    WorldBestRecordOwnerIndex,
    WorldBestRecordUserId,
    WorldBestRecordValue,
    WorldBestRecordPlayInfo
) VALUES (
    ?,
    ?,
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
    .bind(&data.event_schedule_id)
    .bind(&data.last_reward_point)
    .bind(&data.best_record_value)
    .bind(&data.is_possible_quick_reward)
    .bind(&data.world_best_record_owner_index)
    .bind(&data.world_best_record_user_id)
    .bind(&data.world_best_record_value)
    .bind(&data.world_best_record_play_info)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MiniGameInfo>> {
    sqlx::query_as::<_, MiniGameInfo>("SELECT * FROM MiniGameInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameInfo rows for a UID.
pub async fn delete_mini_game_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MiniGameInfo> {
    sqlx::query_as::<_, MiniGameInfo>("SELECT * FROM MiniGameInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MiniGameInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameInfo (
    Uid,
    EventScheduleId,
    LastRewardPoint,
    BestRecordValue,
    IsPossibleQuickReward,
    WorldBestRecordOwnerIndex,
    WorldBestRecordUserId,
    WorldBestRecordValue,
    WorldBestRecordPlayInfo
) VALUES (
    ?,
    ?,
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
    .bind(&data.event_schedule_id)
    .bind(&data.last_reward_point)
    .bind(&data.best_record_value)
    .bind(&data.is_possible_quick_reward)
    .bind(&data.world_best_record_owner_index)
    .bind(&data.world_best_record_user_id)
    .bind(&data.world_best_record_value)
    .bind(&data.world_best_record_play_info)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
