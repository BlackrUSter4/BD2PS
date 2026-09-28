use crate::models::game::cafeteria::cafeteria_regular_costume_note_info::CafeteriaRegularCostumeNoteInfo;
use sqlx::SqlitePool;

/// Add a single CafeteriaRegularCostumeNoteInfo record from a Rust struct.
pub async fn add_cafeteria_regular_costume_note_info(
    pool: &SqlitePool,
    data: &CafeteriaRegularCostumeNoteInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CafeteriaRegularCostumeNoteInfo (
    Uid,
    CostumeId,
    ServeCount,
    IsReceivedReward
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.costume_id)
    .bind(&data.serve_count)
    .bind(&data.is_received_reward)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cafeteria_regular_costume_note_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CafeteriaRegularCostumeNoteInfo>> {
    sqlx::query_as::<_, CafeteriaRegularCostumeNoteInfo>(
        "SELECT * FROM CafeteriaRegularCostumeNoteInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all CafeteriaRegularCostumeNoteInfo rows for a UID.
pub async fn delete_cafeteria_regular_costume_note_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CafeteriaRegularCostumeNoteInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<CafeteriaRegularCostumeNoteInfo> {
    sqlx::query_as::<_, CafeteriaRegularCostumeNoteInfo>(
        "SELECT * FROM CafeteriaRegularCostumeNoteInfo WHERE Uid = ? AND Index = ?",
    )
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
) -> sqlx::Result<Vec<CafeteriaRegularCostumeNoteInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CafeteriaRegularCostumeNoteInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CafeteriaRegularCostumeNoteInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Get one account's note row for a specific costume, if it exists yet.
pub async fn get_by_costume_id(
    pool: &SqlitePool,
    uid: i64,
    costume_id: i32,
) -> sqlx::Result<Option<CafeteriaRegularCostumeNoteInfo>> {
    sqlx::query_as::<_, CafeteriaRegularCostumeNoteInfo>(
        "SELECT * FROM CafeteriaRegularCostumeNoteInfo WHERE Uid = ? AND CostumeId = ?",
    )
    .bind(uid)
    .bind(costume_id)
    .fetch_optional(pool)
    .await
}

/// Increment ServeCount by 1 for a costume, creating the row (ServeCount = 1) if it doesn't
/// exist yet, and return the row afterward.
pub async fn increment_serve_count(
    pool: &SqlitePool,
    uid: i64,
    costume_id: i32,
) -> sqlx::Result<CafeteriaRegularCostumeNoteInfo> {
    match get_by_costume_id(pool, uid, costume_id).await? {
        Some(row) => {
            let new_count = row.serve_count.unwrap_or(0) + 1;
            sqlx::query(
                "UPDATE CafeteriaRegularCostumeNoteInfo SET ServeCount = ? WHERE Uid = ? AND CostumeId = ?",
            )
            .bind(new_count)
            .bind(uid)
            .bind(costume_id)
            .execute(pool)
            .await?;
            Ok(get_by_costume_id(pool, uid, costume_id)
                .await?
                .expect("row was just updated"))
        }
        None => {
            let fresh = CafeteriaRegularCostumeNoteInfo {
                index: 0,
                uid,
                costume_id: Some(costume_id),
                serve_count: Some(1),
                is_received_reward: Some(false),
            };
            add_cafeteria_regular_costume_note_info(pool, &fresh).await?;
            Ok(get_by_costume_id(pool, uid, costume_id)
                .await?
                .expect("row was just inserted"))
        }
    }
}

/// Mark a costume's note reward as claimed.
pub async fn mark_reward_received(
    pool: &SqlitePool,
    uid: i64,
    costume_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE CafeteriaRegularCostumeNoteInfo SET IsReceivedReward = 1 WHERE Uid = ? AND CostumeId = ?",
    )
    .bind(uid)
    .bind(costume_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert and return the rowid (Index)
pub async fn insert(
    pool: &SqlitePool,
    data: &CafeteriaRegularCostumeNoteInfo,
) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CafeteriaRegularCostumeNoteInfo (
    Uid,
    CostumeId,
    ServeCount,
    IsReceivedReward
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.costume_id)
    .bind(&data.serve_count)
    .bind(&data.is_received_reward)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
