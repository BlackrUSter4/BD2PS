use crate::models::game::life::life_chunk_info::LifeChunkInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<LifeChunkInfo>> {
    sqlx::query_as::<_, LifeChunkInfo>("SELECT * FROM LifeChunkInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Grant a chunk to the user if they don't already own it. Returns true if newly granted.
pub async fn add_if_missing(pool: &SqlitePool, uid: i64, chunk_id: i32) -> sqlx::Result<bool> {
    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT \"Index\" FROM LifeChunkInfo WHERE Uid = ? AND ChunkId = ?")
            .bind(uid)
            .bind(chunk_id)
            .fetch_optional(pool)
            .await?;
    if existing.is_some() {
        return Ok(false);
    }
    sqlx::query("INSERT INTO LifeChunkInfo (Uid, ChunkId) VALUES (?, ?)")
        .bind(uid)
        .bind(chunk_id)
        .execute(pool)
        .await?;
    Ok(true)
}
