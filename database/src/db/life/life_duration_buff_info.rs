use crate::models::game::life::life_duration_buff_info::LifeDurationBuffInfo;
use sqlx::SqlitePool;

pub async fn get_active(
    pool: &SqlitePool,
    uid: i64,
    now: i64,
) -> sqlx::Result<Vec<LifeDurationBuffInfo>> {
    sqlx::query_as::<_, LifeDurationBuffInfo>(
        "SELECT * FROM LifeDurationBuffInfo WHERE Uid = ? AND EndTime > ?",
    )
    .bind(uid)
    .bind(now)
    .fetch_all(pool)
    .await
}

pub async fn insert(pool: &SqlitePool, data: &LifeDurationBuffInfo) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO LifeDurationBuffInfo (Uid, ItemId, EndTime) VALUES (?, ?, ?)")
        .bind(data.uid)
        .bind(data.item_id)
        .bind(data.end_time)
        .execute(pool)
        .await?;
    Ok(())
}

/// Housekeeping: drop buffs that have already expired.
pub async fn delete_expired(pool: &SqlitePool, uid: i64, now: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM LifeDurationBuffInfo WHERE Uid = ? AND EndTime <= ?")
        .bind(uid)
        .bind(now)
        .execute(pool)
        .await?;
    Ok(())
}
