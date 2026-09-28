use crate::models::game::life::life_world_object_info::LifeWorldObjectInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<LifeWorldObjectInfo>> {
    sqlx::query_as::<_, LifeWorldObjectInfo>("SELECT * FROM LifeWorldObjectInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_chunk(
    pool: &SqlitePool,
    uid: i64,
    chunk_id: i32,
) -> sqlx::Result<Vec<LifeWorldObjectInfo>> {
    sqlx::query_as::<_, LifeWorldObjectInfo>(
        "SELECT * FROM LifeWorldObjectInfo WHERE Uid = ? AND ChunkId = ?",
    )
    .bind(uid)
    .bind(chunk_id)
    .fetch_all(pool)
    .await
}

pub async fn get_by_object_index(
    pool: &SqlitePool,
    uid: i64,
    chunk_id: i32,
    object_index: i32,
) -> sqlx::Result<Option<LifeWorldObjectInfo>> {
    sqlx::query_as::<_, LifeWorldObjectInfo>(
        "SELECT * FROM LifeWorldObjectInfo WHERE Uid = ? AND ChunkId = ? AND ObjectIndex = ?",
    )
    .bind(uid)
    .bind(chunk_id)
    .bind(object_index)
    .fetch_optional(pool)
    .await
}

/// Insert a new placed object and return its DB row Index.
pub async fn insert(pool: &SqlitePool, data: &LifeWorldObjectInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO LifeWorldObjectInfo (
    Uid, ChunkId, ObjectIndex, ObjectId, X, Y, Rotate, Status, StartTime, EndTime, ParentIndex
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.chunk_id)
    .bind(data.object_index)
    .bind(data.object_id)
    .bind(data.x)
    .bind(data.y)
    .bind(data.rotate)
    .bind(data.status)
    .bind(data.start_time)
    .bind(data.end_time)
    .bind(data.parent_index)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

/// Upsert by (Uid, ChunkId, ObjectIndex) — used for place/deco/status saves where the client
/// resends the full object state rather than a delta.
pub async fn upsert(pool: &SqlitePool, data: &LifeWorldObjectInfo) -> sqlx::Result<i64> {
    if let Some(existing) = get_by_object_index(
        pool,
        data.uid,
        data.chunk_id,
        data.object_index.unwrap_or_default(),
    )
    .await?
    {
        sqlx::query(
            r#"
UPDATE LifeWorldObjectInfo SET
    ObjectId = ?, X = ?, Y = ?, Rotate = ?, Status = ?, StartTime = ?, EndTime = ?, ParentIndex = ?
WHERE "Index" = ?
"#,
        )
        .bind(data.object_id)
        .bind(data.x)
        .bind(data.y)
        .bind(data.rotate)
        .bind(data.status)
        .bind(data.start_time)
        .bind(data.end_time)
        .bind(data.parent_index)
        .bind(existing.index)
        .execute(pool)
        .await?;
        Ok(existing.index)
    } else {
        insert(pool, data).await
    }
}

pub async fn update_position(
    pool: &SqlitePool,
    index: i64,
    x: i32,
    y: i32,
    rotate: Option<i32>,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE LifeWorldObjectInfo SET X = ?, Y = ?, Rotate = ? WHERE \"Index\" = ?")
        .bind(x)
        .bind(y)
        .bind(rotate)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_status(pool: &SqlitePool, index: i64, status: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE LifeWorldObjectInfo SET Status = ? WHERE \"Index\" = ?")
        .bind(status)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_by_object_index(
    pool: &SqlitePool,
    uid: i64,
    chunk_id: i32,
    object_index: i32,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM LifeWorldObjectInfo WHERE Uid = ? AND ChunkId = ? AND ObjectIndex = ?")
        .bind(uid)
        .bind(chunk_id)
        .bind(object_index)
        .execute(pool)
        .await?;
    Ok(())
}
