use crate::models::game::mini::mini_game_sichuan_schedule_info::MiniGameSichuanScheduleInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameSichuanScheduleInfo record from a Rust struct.
pub async fn add_mini_game_sichuan_schedule_info(
    pool: &SqlitePool,
    data: &MiniGameSichuanScheduleInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSichuanScheduleInfo (
    Uid,
    EventScheduleId,
    InfoIndex,
    WorldBestRecordOwnerIndex,
    WorldBestRecordUserId,
    WorldBestRecordValue,
    BestRecordValue,
    IsBlock
) VALUES (
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
    .bind(&data.info_index)
    .bind(&data.world_best_record_owner_index)
    .bind(&data.world_best_record_user_id)
    .bind(&data.world_best_record_value)
    .bind(&data.best_record_value)
    .bind(&data.is_block)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_sichuan_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameSichuanScheduleInfo>> {
    sqlx::query_as::<_, MiniGameSichuanScheduleInfo>(
        "SELECT * FROM MiniGameSichuanScheduleInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameSichuanScheduleInfo rows for a UID.
pub async fn get_by_schedule(pool: &SqlitePool, uid: i64, event_schedule_id: i32) -> sqlx::Result<Option<MiniGameSichuanScheduleInfo>> {
    sqlx::query_as::<_, MiniGameSichuanScheduleInfo>("SELECT * FROM MiniGameSichuanScheduleInfo WHERE Uid = ? AND EventScheduleId = ?")
        .bind(uid)
        .bind(event_schedule_id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_best(pool: &SqlitePool, uid: i64, event_schedule_id: i32, owner_index: i64, user_id: &str, score: f64) -> sqlx::Result<()> {
    if let Some(row) = get_by_schedule(pool, uid, event_schedule_id).await? {
        let mut best = row.best_record_value.unwrap_or(0.0);
        let mut world_owner = row.world_best_record_owner_index;
        let mut world_user = row.world_best_record_user_id.clone();
        let mut world_value = row.world_best_record_value.unwrap_or(0.0);
        if score > best {
            best = score;
        }
        if score > world_value {
            world_value = score;
            world_owner = Some(owner_index);
            world_user = Some(user_id.to_string());
        }
        sqlx::query("UPDATE MiniGameSichuanScheduleInfo SET BestRecordValue = ?, WorldBestRecordValue = ?, WorldBestRecordOwnerIndex = ?, WorldBestRecordUserId = ? WHERE \"Index\" = ?")
            .bind(best)
            .bind(world_value)
            .bind(world_owner)
            .bind(world_user)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_mini_game_sichuan_schedule_info(pool, &MiniGameSichuanScheduleInfo {
            index: 0, uid, event_schedule_id: Some(event_schedule_id), info_index: None,
            world_best_record_owner_index: Some(owner_index), world_best_record_user_id: Some(user_id.to_string()),
            world_best_record_value: Some(score), best_record_value: Some(score), reward_info_index: None, active_info_index: None, is_block: Some(0),
        }).await?;
    }
    Ok(())
}

pub async fn delete_mini_game_sichuan_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSichuanScheduleInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameSichuanScheduleInfo> {
    sqlx::query_as::<_, MiniGameSichuanScheduleInfo>(
        "SELECT * FROM MiniGameSichuanScheduleInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameSichuanScheduleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameSichuanScheduleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameSichuanScheduleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameSichuanScheduleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameSichuanScheduleInfo (
    Uid,
    EventScheduleId,
    InfoIndex,
    WorldBestRecordOwnerIndex,
    WorldBestRecordUserId,
    WorldBestRecordValue,
    BestRecordValue,
    IsBlock
) VALUES (
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
    .bind(&data.info_index)
    .bind(&data.world_best_record_owner_index)
    .bind(&data.world_best_record_user_id)
    .bind(&data.world_best_record_value)
    .bind(&data.best_record_value)
    .bind(&data.is_block)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
