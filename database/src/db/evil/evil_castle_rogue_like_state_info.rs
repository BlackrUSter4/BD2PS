use crate::models::game::evil::evil_castle_rogue_like_state_info::EvilCastleRogueLikeStateInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeStateInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_state_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeStateInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeStateInfo (
    Uid,
    Floor,
    Room,
    State
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.floor)
    .bind(&data.room)
    .bind(&data.state)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_state_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeStateInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeStateInfo>(
        "SELECT * FROM EvilCastleRogueLikeStateInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeStateInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_state_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeStateInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeStateInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeStateInfo>(
        "SELECT * FROM EvilCastleRogueLikeStateInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeStateInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeStateInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeStateInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleRogueLikeStateInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeStateInfo (
    Uid,
    Floor,
    Room,
    State
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.floor)
    .bind(&data.room)
    .bind(&data.state)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_one(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<EvilCastleRogueLikeStateInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeStateInfo>(
        "SELECT * FROM EvilCastleRogueLikeStateInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// One live row per uid — replace it (delete-then-insert).
pub async fn upsert(pool: &SqlitePool, data: &EvilCastleRogueLikeStateInfo) -> sqlx::Result<()> {
    delete_evil_castle_rogue_like_state_info(pool, data.uid).await?;
    insert(pool, data).await.map(|_| ())
}
