use crate::models::game::mini::mini_game_sichuan_info::MiniGameSichuanInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameSichuanInfo record from a Rust struct.
pub async fn add_mini_game_sichuan_info(
    pool: &SqlitePool,
    data: &MiniGameSichuanInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSichuanInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_sichuan_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameSichuanInfo>> {
    sqlx::query_as::<_, MiniGameSichuanInfo>("SELECT * FROM MiniGameSichuanInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameSichuanInfo rows for a UID.
pub async fn get_by_group_id(pool: &SqlitePool, uid: i64, group_id: i32, id: i32) -> sqlx::Result<Option<MiniGameSichuanInfo>> {
    sqlx::query_as::<_, MiniGameSichuanInfo>("SELECT * FROM MiniGameSichuanInfo WHERE Uid = ? AND GroupId = ? AND Id = ?")
        .bind(uid)
        .bind(group_id)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn increment_challenge(pool: &SqlitePool, uid: i64, group_id: i32, id: i32) -> sqlx::Result<()> {
    if let Some(row) = get_by_group_id(pool, uid, group_id, id).await? {
        sqlx::query("UPDATE MiniGameSichuanInfo SET ChallengeIndex = ChallengeIndex + 1 WHERE \"Index\" = ?")
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_mini_game_sichuan_info(pool, &MiniGameSichuanInfo { index: 0, uid, group_id: Some(group_id), id: Some(id), challenge_index: 1 }).await?;
    }
    Ok(())
}

pub async fn delete_mini_game_sichuan_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSichuanInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameSichuanInfo> {
    sqlx::query_as::<_, MiniGameSichuanInfo>(
        "SELECT * FROM MiniGameSichuanInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameSichuanInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameSichuanInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameSichuanInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameSichuanInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameSichuanInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
