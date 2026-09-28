use crate::models::game::mini::mini_game_hopscotch_gallery_info::MiniGameHopscotchGallery;
use sqlx::SqlitePool;

pub async fn list_for_event(
    pool: &SqlitePool,
    uid: i64,
    event_schedule_id: i32,
) -> sqlx::Result<Vec<MiniGameHopscotchGallery>> {
    sqlx::query_as::<_, MiniGameHopscotchGallery>(
        "SELECT * FROM MiniGameHopscotchGallery WHERE Uid = ? AND EventScheduleId = ?",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .fetch_all(pool)
    .await
}

/// Unlocks a gallery entry (no-op if already unlocked). Returns true if this call newly
/// unlocked it (used to decide whether to echo it back in `acquired_gallery_id`).
pub async fn unlock(
    pool: &SqlitePool,
    uid: i64,
    event_schedule_id: i32,
    gallery_id: i32,
) -> sqlx::Result<bool> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO MiniGameHopscotchGallery (Uid, EventScheduleId, GalleryId) VALUES (?, ?, ?)",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(gallery_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
