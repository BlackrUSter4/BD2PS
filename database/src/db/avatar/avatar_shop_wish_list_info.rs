use sqlx::SqlitePool;

pub async fn get_all(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<i32>> {
    let rows: Vec<(i32,)> = sqlx::query_as("SELECT ShopId FROM AvatarShopWishListInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Full replace — the client always sends its complete wishlist, not incremental deltas.
pub async fn save(pool: &SqlitePool, uid: i64, shop_ids: &[i32]) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM AvatarShopWishListInfo WHERE Uid = ?")
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    for id in shop_ids {
        sqlx::query("INSERT INTO AvatarShopWishListInfo (Uid, ShopId) VALUES (?, ?)")
            .bind(uid)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}
