use crate::models::game::char::char_awake_info::CharAwakeInfo;
use sqlx::SqlitePool;

/// Add a single CharAwakeInfo record from a Rust struct.
pub async fn add_char_awake_info(pool: &SqlitePool, data: &CharAwakeInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CharAwakeInfo (
    Uid,
    UniqueCharId,
    IsAwake,
    ImprintSlot1Level,
    ImprintSlot2Level,
    ImprintSlot3Level
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.unique_char_id)
    .bind(&data.is_awake)
    .bind(&data.imprint_slot_1_level)
    .bind(&data.imprint_slot_2_level)
    .bind(&data.imprint_slot_3_level)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_char_awake_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CharAwakeInfo>> {
    sqlx::query_as::<_, CharAwakeInfo>("SELECT * FROM CharAwakeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CharAwakeInfo rows for a UID.
pub async fn delete_char_awake_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CharAwakeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<CharAwakeInfo> {
    sqlx::query_as::<_, CharAwakeInfo>("SELECT * FROM CharAwakeInfo WHERE Uid = ? AND Index = ?")
        .bind(uid)
        .bind(index)
        .fetch_one(pool)
        .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<CharAwakeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CharAwakeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CharAwakeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Get a single record by UniqueCharId.
pub async fn get_by_unique_char_id(
    pool: &SqlitePool,
    uid: i64,
    unique_char_id: i32,
) -> sqlx::Result<Option<CharAwakeInfo>> {
    sqlx::query_as::<_, CharAwakeInfo>(
        "SELECT * FROM CharAwakeInfo WHERE Uid = ? AND UniqueCharId = ?",
    )
    .bind(uid)
    .bind(unique_char_id)
    .fetch_optional(pool)
    .await
}

/// Set the awake flag for a character, creating the row if it doesn't exist yet.
pub async fn set_awake(pool: &SqlitePool, uid: i64, unique_char_id: i32) -> sqlx::Result<()> {
    if get_by_unique_char_id(pool, uid, unique_char_id).await?.is_some() {
        sqlx::query("UPDATE CharAwakeInfo SET IsAwake = 1 WHERE Uid = ? AND UniqueCharId = ?")
            .bind(uid)
            .bind(unique_char_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query(
            "INSERT INTO CharAwakeInfo (Uid, UniqueCharId, IsAwake) VALUES (?, ?, 1)",
        )
        .bind(uid)
        .bind(unique_char_id)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Set one of the three imprint slot levels, creating the row if it doesn't exist yet.
pub async fn set_imprint_slot_level(
    pool: &SqlitePool,
    uid: i64,
    unique_char_id: i32,
    slot: i32,
    level: i32,
) -> sqlx::Result<()> {
    let column = match slot {
        1 => "ImprintSlot1Level",
        2 => "ImprintSlot2Level",
        _ => "ImprintSlot3Level",
    };
    if get_by_unique_char_id(pool, uid, unique_char_id).await?.is_some() {
        let query = format!(
            "UPDATE CharAwakeInfo SET {} = ? WHERE Uid = ? AND UniqueCharId = ?",
            column
        );
        sqlx::query(&query)
            .bind(level)
            .bind(uid)
            .bind(unique_char_id)
            .execute(pool)
            .await?;
    } else {
        let query = format!(
            "INSERT INTO CharAwakeInfo (Uid, UniqueCharId, {}) VALUES (?, ?, ?)",
            column
        );
        sqlx::query(&query)
            .bind(uid)
            .bind(unique_char_id)
            .bind(level)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CharAwakeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CharAwakeInfo (
    Uid,
    UniqueCharId,
    IsAwake,
    ImprintSlot1Level,
    ImprintSlot2Level,
    ImprintSlot3Level
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.unique_char_id)
    .bind(&data.is_awake)
    .bind(&data.imprint_slot_1_level)
    .bind(&data.imprint_slot_2_level)
    .bind(&data.imprint_slot_3_level)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
