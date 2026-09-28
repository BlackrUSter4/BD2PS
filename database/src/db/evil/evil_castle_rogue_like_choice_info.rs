use crate::models::game::evil::evil_castle_rogue_like_choice_info::EvilCastleRogueLikeChoiceInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeChoiceInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_choice_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeChoiceInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeChoiceInfo (
    Uid,
    Type,
    Ids
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.ids)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_choice_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeChoiceInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeChoiceInfo>(
        "SELECT * FROM EvilCastleRogueLikeChoiceInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeChoiceInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_choice_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeChoiceInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeChoiceInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeChoiceInfo>(
        "SELECT * FROM EvilCastleRogueLikeChoiceInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeChoiceInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeChoiceInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeChoiceInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleRogueLikeChoiceInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeChoiceInfo (
    Uid,
    Type,
    Ids
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.ids)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Replace the caller's current choice (delete-then-insert).
pub async fn upsert(
    pool: &SqlitePool,
    uid: i64,
    r#type: i32,
    ids: &[i32],
) -> sqlx::Result<()> {
    delete_evil_castle_rogue_like_choice_info(pool, uid).await?;
    let data = EvilCastleRogueLikeChoiceInfo {
        index: 0,
        uid,
        r#type: Some(r#type),
        ids: Some(serde_json::to_string(ids).unwrap_or_default()),
    };
    insert(pool, &data).await.map(|_| ())
}
