use crate::models::game::avatar::avatar_motion_info::AvatarMotionInfo;
use sqlx::SqlitePool;

pub async fn get_all(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<AvatarMotionInfo>> {
    sqlx::query_as::<_, AvatarMotionInfo>("SELECT * FROM AvatarMotionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Grant ownership of an avatar motion. No-ops if already owned.
pub async fn grant(pool: &SqlitePool, uid: i64, motion_id: i32) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO AvatarMotionInfo (Uid, MotionId) VALUES (?, ?) \
         ON CONFLICT(Uid, MotionId) DO NOTHING",
    )
    .bind(uid)
    .bind(motion_id)
    .execute(pool)
    .await?;
    Ok(())
}
