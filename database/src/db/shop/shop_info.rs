use crate::models::game::shop::shop_info::ShopInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid_and_shop(
    pool: &SqlitePool,
    uid: i64,
    shop_id: i32,
) -> sqlx::Result<Option<ShopInfo>> {
    sqlx::query_as::<_, ShopInfo>("SELECT * FROM ShopInfo WHERE Uid = ? AND ShopId = ?")
        .bind(uid)
        .bind(shop_id)
        .fetch_optional(pool)
        .await
}

pub async fn insert(pool: &SqlitePool, data: &ShopInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO ShopInfo (Uid, ShopId, ShopRemainTime, ShopRandSeed) VALUES (?, ?, ?, ?)",
    )
    .bind(data.uid)
    .bind(data.shop_id)
    .bind(data.shop_remain_time)
    .bind(data.shop_rand_seed)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn update_rotation(
    pool: &SqlitePool,
    uid: i64,
    shop_id: i32,
    remain_time: i32,
    rand_seed: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE ShopInfo SET ShopRemainTime = ?, ShopRandSeed = ? WHERE Uid = ? AND ShopId = ?")
        .bind(remain_time)
        .bind(rand_seed)
        .bind(uid)
        .bind(shop_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get or create the account's rotation row for a shop, regenerating it (real rand seed, real
/// remain-time from ShopTable) if it's new or expired.
pub async fn get_or_refresh(
    pool: &SqlitePool,
    uid: i64,
    shop_id: i32,
    reset_seconds: i32,
) -> sqlx::Result<ShopInfo> {
    if let Some(row) = get_by_uid_and_shop(pool, uid, shop_id).await? {
        if row.shop_remain_time.unwrap_or(0) > 0 {
            return Ok(row);
        }
        let seed: i32 = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) & 0x7fffffff) as i32;
        update_rotation(pool, uid, shop_id, reset_seconds, seed).await?;
        return get_by_uid_and_shop(pool, uid, shop_id).await.map(|r| r.unwrap());
    }

    let seed: i32 = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) & 0x7fffffff) as i32;
    insert(
        pool,
        &ShopInfo { index: 0, uid, shop_id, shop_remain_time: Some(reset_seconds), shop_rand_seed: Some(seed) },
    )
    .await?;
    get_by_uid_and_shop(pool, uid, shop_id).await.map(|r| r.unwrap())
}
