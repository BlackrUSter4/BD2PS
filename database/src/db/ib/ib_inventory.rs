use crate::models::game::ib::ib_inventory::IbInventory;
use sqlx::SqlitePool;

pub async fn list(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<IbInventory>> {
    sqlx::query_as::<_, IbInventory>("SELECT * FROM IbInventory WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<Option<IbInventory>> {
    sqlx::query_as::<_, IbInventory>("SELECT * FROM IbInventory WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .fetch_optional(pool)
        .await
}

pub async fn next_inven_index(pool: &SqlitePool, uid: i64) -> sqlx::Result<i64> {
    let row: (Option<i64>,) = sqlx::query_as("SELECT MAX(InvenIndex) FROM IbInventory WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0.unwrap_or(0) + 1)
}

pub async fn add_item(pool: &SqlitePool, uid: i64, r#type: i32, item_id: i32, level: i32) -> sqlx::Result<i64> {
    let inven_index = next_inven_index(pool, uid).await?;
    sqlx::query("INSERT INTO IbInventory (Uid, InvenIndex, Type, ItemId, Level) VALUES (?, ?, ?, ?, ?)")
        .bind(uid)
        .bind(inven_index)
        .bind(r#type)
        .bind(item_id)
        .bind(level)
        .execute(pool)
        .await?;
    Ok(inven_index)
}

pub async fn remove(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IbInventory WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_level(pool: &SqlitePool, uid: i64, inven_index: i64, level: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE IbInventory SET Level = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(level)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn clear_all_for_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IbInventory WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
