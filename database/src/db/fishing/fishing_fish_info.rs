use crate::models::game::fishing::fishing_fish_info::FishingFishInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FishingFishInfo>> {
    sqlx::query_as::<_, FishingFishInfo>("SELECT * FROM FishingFishInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<i64> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM FishingFishInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// Insert a caught fish, returning its new inventory index.
pub async fn insert(pool: &SqlitePool, uid: i64, fish_id: i32, size: i32, time_value: i64) -> sqlx::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO FishingFishInfo (Uid, FishId, Size, TimeValue, IsLock) VALUES (?, ?, ?, ?, 0)",
    )
    .bind(uid)
    .bind(fish_id)
    .bind(size)
    .bind(time_value)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn set_lock(pool: &SqlitePool, uid: i64, inven_index: i64, is_lock: bool) -> sqlx::Result<()> {
    sqlx::query("UPDATE FishingFishInfo SET IsLock = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(is_lock)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Remove a caught fish from inventory (e.g. sold via the shop).
pub async fn delete(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FishingFishInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}
