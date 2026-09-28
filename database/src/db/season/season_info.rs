use crate::models::game::season::season_info::SeasonInfo;
use sqlx::SqlitePool;

/// Add a single SeasonInfo record from a Rust struct.
pub async fn add_season_info(pool: &SqlitePool, data: &SeasonInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SeasonInfo (
    Uid,
    Season,
    StartTime,
    EndTime,
    ErrorFlag,
    ReturnFlag,
    RankRewardGroupId
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
    .bind(&data.season)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.error_flag)
    .bind(&data.return_flag)
    .bind(&data.rank_reward_group_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_season_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<SeasonInfo>> {
    sqlx::query_as::<_, SeasonInfo>("SELECT * FROM SeasonInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all SeasonInfo rows for a UID.
pub async fn delete_season_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SeasonInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<SeasonInfo> {
    sqlx::query_as::<_, SeasonInfo>("SELECT * FROM SeasonInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<SeasonInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM SeasonInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, SeasonInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &SeasonInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO SeasonInfo (
    Uid,
    Season,
    StartTime,
    EndTime,
    ErrorFlag,
    ReturnFlag,
    RankRewardGroupId
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
    .bind(&data.season)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.error_flag)
    .bind(&data.return_flag)
    .bind(&data.rank_reward_group_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
