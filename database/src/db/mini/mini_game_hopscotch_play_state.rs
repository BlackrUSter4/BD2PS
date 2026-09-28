use crate::models::game::mini::mini_game_hopscotch_play_state::MiniGameHopscotchPlayState;
use sqlx::SqlitePool;

pub async fn set(pool: &SqlitePool, uid: i64, event_schedule_id: i32, stage_id: i32) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameHopscotchPlayState (Uid, EventScheduleId, StageId) VALUES (?, ?, ?)
ON CONFLICT (Uid) DO UPDATE SET EventScheduleId = excluded.EventScheduleId, StageId = excluded.StageId
"#,
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(stage_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<MiniGameHopscotchPlayState>> {
    sqlx::query_as::<_, MiniGameHopscotchPlayState>(
        "SELECT * FROM MiniGameHopscotchPlayState WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}
