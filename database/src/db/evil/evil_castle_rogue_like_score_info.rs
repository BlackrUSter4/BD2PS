use crate::models::game::evil::evil_castle_rogue_like_score_info::EvilCastleRogueLikeScoreInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeScoreInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_score_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeScoreInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeScoreInfo (
    Uid,
    TotalScore,
    Obsidian,
    AllUserTotalScore,
    MaxTryLevel,
    MaxRewardLevel,
    CrystalDamage
) VALUES (
    ?,
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
    .bind(&data.total_score)
    .bind(&data.obsidian)
    .bind(&data.all_user_total_score)
    .bind(&data.max_try_level)
    .bind(&data.max_reward_level)
    .bind(&data.crystal_damage)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_score_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeScoreInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeScoreInfo>(
        "SELECT * FROM EvilCastleRogueLikeScoreInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeScoreInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_score_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeScoreInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeScoreInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeScoreInfo>(
        "SELECT * FROM EvilCastleRogueLikeScoreInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeScoreInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeScoreInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeScoreInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleRogueLikeScoreInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeScoreInfo (
    Uid,
    TotalScore,
    Obsidian,
    AllUserTotalScore,
    MaxTryLevel,
    MaxRewardLevel,
    CrystalDamage
) VALUES (
    ?,
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
    .bind(&data.total_score)
    .bind(&data.obsidian)
    .bind(&data.all_user_total_score)
    .bind(&data.max_try_level)
    .bind(&data.max_reward_level)
    .bind(&data.crystal_damage)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_one(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<EvilCastleRogueLikeScoreInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeScoreInfo>(
        "SELECT * FROM EvilCastleRogueLikeScoreInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// One live row per uid (current season's score) — replace it.
pub async fn upsert(pool: &SqlitePool, data: &EvilCastleRogueLikeScoreInfo) -> sqlx::Result<()> {
    delete_evil_castle_rogue_like_score_info(pool, data.uid).await?;
    insert(pool, data).await.map(|_| ())
}

/// Top accounts by total_score, real cross-account aggregation.
pub async fn top_by_score(pool: &SqlitePool, limit: i64) -> sqlx::Result<Vec<EvilCastleRogueLikeScoreInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeScoreInfo>(
        "SELECT * FROM EvilCastleRogueLikeScoreInfo ORDER BY TotalScore DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// This account's rank among everyone (1-based).
pub async fn rank_for(pool: &SqlitePool, uid: i64) -> sqlx::Result<i64> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) + 1 FROM EvilCastleRogueLikeScoreInfo WHERE TotalScore > (
            SELECT COALESCE(MAX(TotalScore), 0) FROM EvilCastleRogueLikeScoreInfo WHERE Uid = ?
        )",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Cross-account total, computed live rather than cached.
pub async fn sum_all_user_total_score(pool: &SqlitePool) -> sqlx::Result<i64> {
    let row: (Option<i64>,) =
        sqlx::query_as("SELECT SUM(TotalScore) FROM EvilCastleRogueLikeScoreInfo")
            .fetch_one(pool)
            .await?;
    Ok(row.0.unwrap_or(0))
}
