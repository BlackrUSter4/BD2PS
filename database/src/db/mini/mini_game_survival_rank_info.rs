use crate::models::game::mini::mini_game_survival_rank_info::MiniGameSurvivalRankInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameSurvivalRankInfo record from a Rust struct.
pub async fn add_mini_game_survival_rank_info(
    pool: &SqlitePool,
    data: &MiniGameSurvivalRankInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSurvivalRankInfo (
    Uid,
    Rank,
    OwnerIndex,
    UserId,
    Point
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.point)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_survival_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameSurvivalRankInfo>> {
    sqlx::query_as::<_, MiniGameSurvivalRankInfo>(
        "SELECT * FROM MiniGameSurvivalRankInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameSurvivalRankInfo rows for a UID.
pub async fn get_own(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<MiniGameSurvivalRankInfo>> {
    sqlx::query_as::<_, MiniGameSurvivalRankInfo>("SELECT * FROM MiniGameSurvivalRankInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_best(pool: &SqlitePool, uid: i64, owner_index: i64, user_id: &str, point: i64) -> sqlx::Result<()> {
    if let Some(row) = get_own(pool, uid).await? {
        if point > row.point.unwrap_or(0) {
            sqlx::query("UPDATE MiniGameSurvivalRankInfo SET Point = ? WHERE \"Index\" = ?")
                .bind(point)
                .bind(row.index)
                .execute(pool)
                .await?;
        }
    } else {
        add_mini_game_survival_rank_info(pool, &MiniGameSurvivalRankInfo { index: 0, uid, rank: None, owner_index: Some(owner_index), user_id: Some(user_id.to_string()), point: Some(point) }).await?;
    }
    Ok(())
}

pub async fn get_top(pool: &SqlitePool, limit: i64) -> sqlx::Result<Vec<MiniGameSurvivalRankInfo>> {
    sqlx::query_as::<_, MiniGameSurvivalRankInfo>("SELECT * FROM MiniGameSurvivalRankInfo ORDER BY Point DESC LIMIT ?")
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn delete_mini_game_survival_rank_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSurvivalRankInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameSurvivalRankInfo> {
    sqlx::query_as::<_, MiniGameSurvivalRankInfo>(
        "SELECT * FROM MiniGameSurvivalRankInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameSurvivalRankInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameSurvivalRankInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameSurvivalRankInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameSurvivalRankInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameSurvivalRankInfo (
    Uid,
    Rank,
    OwnerIndex,
    UserId,
    Point
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.point)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
