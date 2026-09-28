use crate::models::game::spine_interaction::spine_interaction_achievement::SpineInteractionAchievement;
use sqlx::SqlitePool;

pub async fn list_for_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<SpineInteractionAchievement>> {
    sqlx::query_as::<_, SpineInteractionAchievement>(
        "SELECT * FROM SpineInteractionAchievement WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Unlocks are permanent — never deletes existing rows, only adds new ones. The client resaves
/// its full known state each time (SpineInteractionAchievementSaveRequest), so this is
/// intentionally upsert-only rather than a destructive replace.
pub async fn unlock(
    pool: &SqlitePool,
    uid: i64,
    interaction_group_id: i32,
    group_id: i32,
    point_id: i32,
    motion_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT OR IGNORE INTO SpineInteractionAchievement
    (Uid, InteractionGroupId, GroupId, PointId, MotionId)
VALUES (?, ?, ?, ?, ?)
"#,
    )
    .bind(uid)
    .bind(interaction_group_id)
    .bind(group_id)
    .bind(point_id)
    .bind(motion_id)
    .execute(pool)
    .await?;
    Ok(())
}
