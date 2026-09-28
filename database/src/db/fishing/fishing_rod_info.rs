use crate::models::game::fishing::fishing_rod_info::FishingRodInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FishingRodInfo>> {
    sqlx::query_as::<_, FishingRodInfo>("SELECT * FROM FishingRodInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_index(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<Option<FishingRodInfo>> {
    sqlx::query_as::<_, FishingRodInfo>("SELECT * FROM FishingRodInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .fetch_optional(pool)
        .await
}

/// Grant a rod, returning its new inventory index. New accounts start with one starter rod
/// (rod_id = 1 — no FishingRodTable was captured to look up a real starter id, flagged as a
/// placeholder) via this same function.
pub async fn insert(pool: &SqlitePool, uid: i64, rod_id: i32, time_value: Option<i64>) -> sqlx::Result<i64> {
    let result = sqlx::query("INSERT INTO FishingRodInfo (Uid, RodId, TimeValue) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(rod_id)
        .bind(time_value)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}
