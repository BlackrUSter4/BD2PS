use crate::models::game::shop::shop_product_info::ShopProductInfo;
use sqlx::SqlitePool;

pub async fn get_by_shop(
    pool: &SqlitePool,
    uid: i64,
    shop_id: i32,
) -> sqlx::Result<Vec<ShopProductInfo>> {
    sqlx::query_as::<_, ShopProductInfo>("SELECT * FROM ShopProductInfo WHERE Uid = ? AND ShopId = ?")
        .bind(uid)
        .bind(shop_id)
        .fetch_all(pool)
        .await
}

pub async fn add_buy_count(
    pool: &SqlitePool,
    uid: i64,
    shop_id: i32,
    product_id: i32,
    delta: i32,
) -> sqlx::Result<()> {
    let existing = sqlx::query_as::<_, ShopProductInfo>(
        "SELECT * FROM ShopProductInfo WHERE Uid = ? AND ShopId = ? AND ProductId = ?",
    )
    .bind(uid)
    .bind(shop_id)
    .bind(product_id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = existing {
        sqlx::query("UPDATE ShopProductInfo SET BuyCount = BuyCount + ? WHERE \"Index\" = ?")
            .bind(delta)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query(
            "INSERT INTO ShopProductInfo (Uid, ShopId, ProductId, BuyCount) VALUES (?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(shop_id)
        .bind(product_id)
        .bind(delta)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Reset every product's buy count for this account's shop rotation (called on rotation
/// refresh).
pub async fn reset_shop(pool: &SqlitePool, uid: i64, shop_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ShopProductInfo WHERE Uid = ? AND ShopId = ?")
        .bind(uid)
        .bind(shop_id)
        .execute(pool)
        .await?;
    Ok(())
}
