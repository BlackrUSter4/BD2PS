use crate::models::game::ib::ib_shop::IbShop;
use sqlx::SqlitePool;

pub async fn list(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<IbShop>> {
    sqlx::query_as::<_, IbShop>("SELECT * FROM IbShop WHERE Uid = ? ORDER BY Slot")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get(pool: &SqlitePool, uid: i64, slot: i32) -> sqlx::Result<Option<IbShop>> {
    sqlx::query_as::<_, IbShop>("SELECT * FROM IbShop WHERE Uid = ? AND Slot = ?")
        .bind(uid)
        .bind(slot)
        .fetch_optional(pool)
        .await
}

pub async fn replace_all(pool: &SqlitePool, uid: i64, items: &[IbShop]) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM IbShop WHERE Uid = ?").bind(uid).execute(&mut *tx).await?;
    for item in items {
        sqlx::query(
            "INSERT INTO IbShop (Uid, Slot, Type, ItemId, Price, OriginalPrice, IsDiscount, IsReserved, IsSoldOut) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(item.slot)
        .bind(item.r#type)
        .bind(item.item_id)
        .bind(item.price)
        .bind(item.original_price)
        .bind(item.is_discount)
        .bind(item.is_reserved)
        .bind(item.is_sold_out)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn set_reserved(pool: &SqlitePool, uid: i64, slot: i32, is_reserved: bool) -> sqlx::Result<()> {
    sqlx::query("UPDATE IbShop SET IsReserved = ? WHERE Uid = ? AND Slot = ?")
        .bind(is_reserved)
        .bind(uid)
        .bind(slot)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_sold_out(pool: &SqlitePool, uid: i64, slot: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE IbShop SET IsSoldOut = 1 WHERE Uid = ? AND Slot = ?")
        .bind(uid)
        .bind(slot)
        .execute(pool)
        .await?;
    Ok(())
}
