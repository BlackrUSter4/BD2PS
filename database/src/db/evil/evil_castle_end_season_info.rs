use crate::models::game::evil::evil_castle_end_season_info::EvilCastleEndSeasonInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleEndSeasonInfo record from a Rust struct.
pub async fn add_evil_castle_end_season_info(
    pool: &SqlitePool,
    data: &EvilCastleEndSeasonInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleEndSeasonInfo (
    Uid,
    Rank,
    StageIndex,
    Point,
    IsRewarded,
    RewardInfoIndex
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
    .bind(&data.rank)
    .bind(&data.stage_index)
    .bind(&data.point)
    .bind(&data.is_rewarded)
    .bind(&data.reward_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_end_season_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleEndSeasonInfo>> {
    sqlx::query_as::<_, EvilCastleEndSeasonInfo>(
        "SELECT * FROM EvilCastleEndSeasonInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleEndSeasonInfo rows for a UID.
pub async fn delete_evil_castle_end_season_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleEndSeasonInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleEndSeasonInfo> {
    sqlx::query_as::<_, EvilCastleEndSeasonInfo>(
        "SELECT * FROM EvilCastleEndSeasonInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleEndSeasonInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleEndSeasonInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleEndSeasonInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleEndSeasonInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleEndSeasonInfo (
    Uid,
    Rank,
    StageIndex,
    Point,
    IsRewarded,
    RewardInfoIndex
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
    .bind(&data.rank)
    .bind(&data.stage_index)
    .bind(&data.point)
    .bind(&data.is_rewarded)
    .bind(&data.reward_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Mark the most recent unrewarded end-season row for this uid claimed.
pub async fn mark_latest_rewarded(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE EvilCastleEndSeasonInfo SET IsRewarded = 1 WHERE Index = (
            SELECT Index FROM EvilCastleEndSeasonInfo WHERE Uid = ? AND IsRewarded = 0
            ORDER BY Index DESC LIMIT 1
        )",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}
