use crate::models::game::life::life_collection_info::LifeCollectionInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<LifeCollectionInfo>> {
    sqlx::query_as::<_, LifeCollectionInfo>("SELECT * FROM LifeCollectionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Unlock a collection entry if not already unlocked. Returns true if newly unlocked.
pub async fn add_if_missing(pool: &SqlitePool, uid: i64, collection_id: i32) -> sqlx::Result<bool> {
    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT \"Index\" FROM LifeCollectionInfo WHERE Uid = ? AND CollectionId = ?")
            .bind(uid)
            .bind(collection_id)
            .fetch_optional(pool)
            .await?;
    if existing.is_some() {
        return Ok(false);
    }
    sqlx::query("INSERT INTO LifeCollectionInfo (Uid, CollectionId) VALUES (?, ?)")
        .bind(uid)
        .bind(collection_id)
        .execute(pool)
        .await?;
    Ok(true)
}
