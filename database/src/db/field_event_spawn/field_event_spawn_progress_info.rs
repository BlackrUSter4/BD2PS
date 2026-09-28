use crate::models::game::field_event_spawn::field_event_spawn_progress_info::FieldEventSpawnProgressInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> Option<FieldEventSpawnProgressInfo> {
    sqlx::query_as::<_, FieldEventSpawnProgressInfo>("SELECT * FROM FieldEventSpawnProgressInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

pub async fn save(pool: &SqlitePool, info: &FieldEventSpawnProgressInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FieldEventSpawnProgressInfo (Uid, StartTime, EventScheduleId, SpawnEventId, GroupId, CaughtInfo)
VALUES (?, ?, ?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET
    StartTime = excluded.StartTime,
    EventScheduleId = excluded.EventScheduleId,
    SpawnEventId = excluded.SpawnEventId,
    GroupId = excluded.GroupId,
    CaughtInfo = excluded.CaughtInfo
"#,
    )
    .bind(info.uid)
    .bind(info.start_time)
    .bind(info.event_schedule_id)
    .bind(info.spawn_event_id)
    .bind(info.group_id)
    .bind(&info.caught_info)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn clear(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FieldEventSpawnProgressInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
