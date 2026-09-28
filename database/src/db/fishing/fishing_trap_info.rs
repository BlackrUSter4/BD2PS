use crate::models::game::fishing::fishing_trap_info::FishingTrapInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FishingTrapInfo>> {
    sqlx::query_as::<_, FishingTrapInfo>("SELECT * FROM FishingTrapInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn add(pool: &SqlitePool, uid: i64, fish_id: i32, size: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO FishingTrapInfo (Uid, FishId, Size) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(fish_id)
        .bind(size)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn clear_all(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FishingTrapInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
