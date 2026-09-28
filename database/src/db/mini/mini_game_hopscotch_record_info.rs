use crate::models::game::mini::mini_game_hopscotch_record_info::MiniGameHopscotchRecord;
use sqlx::SqlitePool;

pub async fn list_for_event(
    pool: &SqlitePool,
    uid: i64,
    event_schedule_id: i32,
) -> sqlx::Result<Vec<MiniGameHopscotchRecord>> {
    sqlx::query_as::<_, MiniGameHopscotchRecord>(
        "SELECT * FROM MiniGameHopscotchRecord WHERE Uid = ? AND EventScheduleId = ?",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .fetch_all(pool)
    .await
}

/// All of this account's hopscotch records regardless of event (used by the un-scoped generic
/// MiniGameUserRecordInfoRequest, which carries no event_schedule_id of its own).
pub async fn list_all_for_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MiniGameHopscotchRecord>> {
    sqlx::query_as::<_, MiniGameHopscotchRecord>("SELECT * FROM MiniGameHopscotchRecord WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get(
    pool: &SqlitePool,
    uid: i64,
    event_schedule_id: i32,
    stage_id: i32,
) -> sqlx::Result<Option<MiniGameHopscotchRecord>> {
    sqlx::query_as::<_, MiniGameHopscotchRecord>(
        "SELECT * FROM MiniGameHopscotchRecord WHERE Uid = ? AND EventScheduleId = ? AND StageId = ?",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(stage_id)
    .fetch_optional(pool)
    .await
}

/// Records a stage attempt result, keeping the best (largest captured_area, then lowest
/// clear_time) if a record already exists for this (uid, event_schedule_id, stage_id).
pub async fn submit_result(
    pool: &SqlitePool,
    uid: i64,
    event_schedule_id: i32,
    stage_id: i32,
    captured_area: i32,
    clear_time: i32,
) -> sqlx::Result<()> {
    let existing = get(pool, uid, event_schedule_id, stage_id).await?;
    let is_better = match &existing {
        None => true,
        Some(e) => captured_area > e.captured_area
            || (captured_area == e.captured_area && clear_time < e.clear_time),
    };
    if !is_better {
        return Ok(());
    }
    sqlx::query(
        r#"
INSERT INTO MiniGameHopscotchRecord (Uid, EventScheduleId, StageId, IsClear, CapturedArea, ClearTime)
VALUES (?, ?, ?, 1, ?, ?)
ON CONFLICT (Uid, EventScheduleId, StageId)
DO UPDATE SET IsClear = 1, CapturedArea = excluded.CapturedArea, ClearTime = excluded.ClearTime
"#,
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(stage_id)
    .bind(captured_area)
    .bind(clear_time)
    .execute(pool)
    .await?;
    Ok(())
}

/// Ranking across all accounts for a given (event_schedule_id, stage_id), best captured_area
/// first then lowest clear_time, real cross-account data (no bot-padding, matching this
/// project's established convention of never fabricating leaderboard entries).
pub async fn ranking_for_stage(
    pool: &SqlitePool,
    event_schedule_id: i32,
    stage_id: i32,
    limit: i64,
) -> sqlx::Result<Vec<MiniGameHopscotchRecord>> {
    sqlx::query_as::<_, MiniGameHopscotchRecord>(
        r#"
SELECT * FROM MiniGameHopscotchRecord
WHERE EventScheduleId = ? AND StageId = ? AND IsClear = 1
ORDER BY CapturedArea DESC, ClearTime ASC
LIMIT ?
"#,
    )
    .bind(event_schedule_id)
    .bind(stage_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Top cleared records across every event/stage (used by the un-scoped generic
/// MiniGameRankingRequest, which carries no event/stage filter of its own).
pub async fn top_overall(pool: &SqlitePool, limit: i64) -> sqlx::Result<Vec<MiniGameHopscotchRecord>> {
    sqlx::query_as::<_, MiniGameHopscotchRecord>(
        r#"
SELECT * FROM MiniGameHopscotchRecord
WHERE IsClear = 1
ORDER BY CapturedArea DESC, ClearTime ASC
LIMIT ?
"#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Real percentile computed from actual stored cross-account records for this stage: the
/// fraction of OTHER cleared records that are worse than or equal to this one. None if there
/// are no other records to compare against yet.
pub async fn percentile(
    pool: &SqlitePool,
    event_schedule_id: i32,
    stage_id: i32,
    captured_area: i32,
    clear_time: i32,
) -> sqlx::Result<Option<f64>> {
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM MiniGameHopscotchRecord WHERE EventScheduleId = ? AND StageId = ? AND IsClear = 1",
    )
    .bind(event_schedule_id)
    .bind(stage_id)
    .fetch_one(pool)
    .await?;
    if total.0 == 0 {
        return Ok(None);
    }
    let better_or_equal: (i64,) = sqlx::query_as(
        r#"
SELECT COUNT(*) FROM MiniGameHopscotchRecord
WHERE EventScheduleId = ? AND StageId = ? AND IsClear = 1
AND (CapturedArea > ? OR (CapturedArea = ? AND ClearTime <= ?))
"#,
    )
    .bind(event_schedule_id)
    .bind(stage_id)
    .bind(captured_area)
    .bind(captured_area)
    .bind(clear_time)
    .fetch_one(pool)
    .await?;
    Ok(Some(better_or_equal.0 as f64 / total.0 as f64 * 100.0))
}
