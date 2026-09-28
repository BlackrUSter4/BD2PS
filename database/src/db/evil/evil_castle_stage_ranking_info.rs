use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::evil::evil_castle_stage_ranking_info::EvilCastleStageRankingInfo;
/// Insert a full JSON array of EvilCastleStageRankingInfo records for a UID.
pub async fn insert_evil_castle_stage_ranking_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("evilCastleStageRankingInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_evil_castle_stage_ranking_info: missing or invalid 'evilCastleStageRankingInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let rank = entry
            .get("rank")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let point = entry
            .get("point")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let total_rank = entry
            .get("totalRank")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let total_point = entry
            .get("totalPoint")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO EvilCastleStageRankingInfo (
    Uid,
    Rank,
    Point,
    TotalRank,
    TotalPoint
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&rank)
        .bind(&point)
        .bind(&total_rank)
        .bind(&total_point)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EvilCastleStageRankingInfo record from a Rust struct.
pub async fn add_evil_castle_stage_ranking_info(pool: &SqlitePool, data: &EvilCastleStageRankingInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleStageRankingInfo (
    Uid,
    Rank,
    Point,
    TotalRank,
    TotalPoint
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.rank)
    .bind(&data.point)
    .bind(&data.total_rank)
    .bind(&data.total_point)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_stage_ranking_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<EvilCastleStageRankingInfo>> {
    sqlx::query_as::<_, EvilCastleStageRankingInfo>("SELECT * FROM EvilCastleStageRankingInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EvilCastleStageRankingInfo rows for a UID.
pub async fn delete_evil_castle_stage_ranking_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleStageRankingInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}