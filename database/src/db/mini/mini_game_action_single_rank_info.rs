use crate::models::game::mini::mini_game_action_single_rank_info::MiniGameActionSingleRankInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameActionSingleRankInfo record from a Rust struct.
pub async fn add_mini_game_action_single_rank_info(
    pool: &SqlitePool,
    data: &MiniGameActionSingleRankInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameActionSingleRankInfo (
    Uid,
    OwnerIndex,
    UserId,
    Rank,
    Score,
    CharId
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
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.rank)
    .bind(&data.score)
    .bind(&data.char_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_action_single_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameActionSingleRankInfo>> {
    sqlx::query_as::<_, MiniGameActionSingleRankInfo>(
        "SELECT * FROM MiniGameActionSingleRankInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameActionSingleRankInfo rows for a UID.
pub async fn get_own(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<MiniGameActionSingleRankInfo>> {
    sqlx::query_as::<_, MiniGameActionSingleRankInfo>("SELECT * FROM MiniGameActionSingleRankInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_best(pool: &SqlitePool, uid: i64, owner_index: i64, user_id: &str, score: f64, char_id: i32) -> sqlx::Result<()> {
    if let Some(row) = get_own(pool, uid).await? {
        if score > row.score.unwrap_or(0.0) {
            sqlx::query("UPDATE MiniGameActionSingleRankInfo SET Score = ?, CharId = ? WHERE \"Index\" = ?")
                .bind(score)
                .bind(char_id)
                .bind(row.index)
                .execute(pool)
                .await?;
        }
    } else {
        add_mini_game_action_single_rank_info(pool, &MiniGameActionSingleRankInfo { index: 0, uid, owner_index: Some(owner_index), user_id: Some(user_id.to_string()), rank: None, score: Some(score), char_id: Some(char_id) }).await?;
    }
    Ok(())
}

pub async fn get_top(pool: &SqlitePool, limit: i64) -> sqlx::Result<Vec<MiniGameActionSingleRankInfo>> {
    sqlx::query_as::<_, MiniGameActionSingleRankInfo>("SELECT * FROM MiniGameActionSingleRankInfo ORDER BY Score DESC LIMIT ?")
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn delete_mini_game_action_single_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameActionSingleRankInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameActionSingleRankInfo> {
    sqlx::query_as::<_, MiniGameActionSingleRankInfo>(
        "SELECT * FROM MiniGameActionSingleRankInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameActionSingleRankInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameActionSingleRankInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameActionSingleRankInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameActionSingleRankInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameActionSingleRankInfo (
    Uid,
    OwnerIndex,
    UserId,
    Rank,
    Score,
    CharId
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
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.rank)
    .bind(&data.score)
    .bind(&data.char_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
