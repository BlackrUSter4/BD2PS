use crate::models::game::life::life_shop_buy_info::LifeShopBuyInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<LifeShopBuyInfo>> {
    sqlx::query_as::<_, LifeShopBuyInfo>("SELECT * FROM LifeShopBuyInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_one(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    shop_id: i32,
) -> sqlx::Result<Option<LifeShopBuyInfo>> {
    sqlx::query_as::<_, LifeShopBuyInfo>(
        "SELECT * FROM LifeShopBuyInfo WHERE Uid = ? AND GroupId = ? AND ShopId = ?",
    )
    .bind(uid)
    .bind(group_id)
    .bind(shop_id)
    .fetch_optional(pool)
    .await
}

pub async fn increment_buy_count(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    shop_id: i32,
    amount: i32,
) -> sqlx::Result<()> {
    if let Some(existing) = get_one(pool, uid, group_id, shop_id).await? {
        sqlx::query("UPDATE LifeShopBuyInfo SET BuyCount = BuyCount + ? WHERE \"Index\" = ?")
            .bind(amount)
            .bind(existing.index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query(
            "INSERT INTO LifeShopBuyInfo (Uid, GroupId, ShopId, BuyCount) VALUES (?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(group_id)
        .bind(shop_id)
        .bind(amount)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Reset (zero out) all buy counts for a UID — called when a reset period rolls over.
pub async fn reset_all(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM LifeShopBuyInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
