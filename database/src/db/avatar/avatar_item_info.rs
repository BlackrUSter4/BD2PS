use crate::models::game::avatar::avatar_item_info::AvatarItemInfo;
use sqlx::SqlitePool;

pub async fn get_all(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<AvatarItemInfo>> {
    sqlx::query_as::<_, AvatarItemInfo>("SELECT * FROM AvatarItemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Grant ownership of an avatar item. No-ops if already owned (avatar items are unique
/// unlocks, not stackable currency).
pub async fn grant(pool: &SqlitePool, uid: i64, item_category: i32, item_id: i32) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO AvatarItemInfo (Uid, ItemCategory, ItemId) VALUES (?, ?, ?) \
         ON CONFLICT(Uid, ItemCategory, ItemId) DO NOTHING",
    )
    .bind(uid)
    .bind(item_category)
    .bind(item_id)
    .execute(pool)
    .await?;
    Ok(())
}
