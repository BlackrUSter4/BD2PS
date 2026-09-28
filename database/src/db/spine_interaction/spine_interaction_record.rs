use crate::models::game::spine_interaction::spine_interaction_record::SpineInteractionRecord;
use sqlx::SqlitePool;

pub async fn list_for_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<SpineInteractionRecord>> {
    sqlx::query_as::<_, SpineInteractionRecord>("SELECT * FROM SpineInteractionRecord WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
) -> sqlx::Result<Option<SpineInteractionRecord>> {
    sqlx::query_as::<_, SpineInteractionRecord>(
        "SELECT * FROM SpineInteractionRecord WHERE Uid = ? AND InvenIndex = ?",
    )
    .bind(uid)
    .bind(inven_index)
    .fetch_optional(pool)
    .await
}

/// Used only by the public (unauthenticated, see auth middleware's allowlist)
/// SpineInteractionRecordData file-serving route, which stands in for a real S3 URL fetch and so
/// carries no session/uid to scope by — inven_index (the SQLite rowid) is already a globally
/// unique, effectively-unguessable-enough handle for that purpose on a private single-deployment
/// server.
pub async fn get_by_index_only(
    pool: &SqlitePool,
    inven_index: i64,
) -> sqlx::Result<Option<SpineInteractionRecord>> {
    sqlx::query_as::<_, SpineInteractionRecord>("SELECT * FROM SpineInteractionRecord WHERE InvenIndex = ?")
        .bind(inven_index)
        .fetch_optional(pool)
        .await
}

/// Inserts a new record and returns its assigned InvenIndex (the SQLite rowid).
pub async fn insert(pool: &SqlitePool, uid: i64, id: i32, record_data: &[u8]) -> sqlx::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO SpineInteractionRecord (Uid, Id, Name, RecordData) VALUES (?, ?, '', ?)",
    )
    .bind(uid)
    .bind(id)
    .bind(record_data)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn update_data(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    record_data: &[u8],
) -> sqlx::Result<bool> {
    let result = sqlx::query(
        "UPDATE SpineInteractionRecord SET RecordData = ? WHERE Uid = ? AND InvenIndex = ?",
    )
    .bind(record_data)
    .bind(uid)
    .bind(inven_index)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn update_name(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    name: &str,
) -> sqlx::Result<bool> {
    let result = sqlx::query("UPDATE SpineInteractionRecord SET Name = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(name)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM SpineInteractionRecord WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
