use crate::models::game::fishing::fishing_item_info::FishingItemInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FishingItemInfo>> {
    sqlx::query_as::<_, FishingItemInfo>("SELECT * FROM FishingItemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

async fn find_stack(pool: &SqlitePool, uid: i64, item_id: i32) -> sqlx::Result<Option<FishingItemInfo>> {
    sqlx::query_as::<_, FishingItemInfo>("SELECT * FROM FishingItemInfo WHERE Uid = ? AND ItemId = ?")
        .bind(uid)
        .bind(item_id)
        .fetch_optional(pool)
        .await
}

/// Grant (or stack onto an existing) fishing-specific item (e.g. bait).
pub async fn grant(pool: &SqlitePool, uid: i64, item_id: i32, item_type: i32, count: i32) -> sqlx::Result<()> {
    if let Some(existing) = find_stack(pool, uid, item_id).await? {
        sqlx::query("UPDATE FishingItemInfo SET Count = Count + ? WHERE InvenIndex = ?")
            .bind(count)
            .bind(existing.inven_index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query(
            "INSERT INTO FishingItemInfo (Uid, ItemId, ItemType, Count, TimeValue) VALUES (?, ?, ?, ?, NULL)",
        )
        .bind(uid)
        .bind(item_id)
        .bind(item_type)
        .bind(count)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Try to consume `count` of an item; returns false (consuming nothing) if not enough is held.
pub async fn consume(pool: &SqlitePool, uid: i64, item_id: i32, count: i32) -> sqlx::Result<bool> {
    let Some(existing) = find_stack(pool, uid, item_id).await? else {
        return Ok(false);
    };
    if existing.count < count {
        return Ok(false);
    }
    if existing.count == count {
        sqlx::query("DELETE FROM FishingItemInfo WHERE InvenIndex = ?")
            .bind(existing.inven_index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE FishingItemInfo SET Count = Count - ? WHERE InvenIndex = ?")
            .bind(count)
            .bind(existing.inven_index)
            .execute(pool)
            .await?;
    }
    Ok(true)
}
