use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::evil::evil_castle_rogue_like_all_user_score_info::EvilCastleRogueLikeAllUserScoreInfo;
/// Insert a full JSON array of EvilCastleRogueLikeAllUserScoreInfo records for a UID.
pub async fn insert_evil_castle_rogue_like_all_user_score_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("evilCastleRogueLikeAllUserScoreInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_evil_castle_rogue_like_all_user_score_info: missing or invalid 'evilCastleRogueLikeAllUserScoreInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let all_user_total_score = entry
            .get("allUserTotalScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        sqlx::query(
            r#"
INSERT INTO EvilCastleRogueLikeAllUserScoreInfo (
    Uid,
    AllUserTotalScore
) VALUES (
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&all_user_total_score)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EvilCastleRogueLikeAllUserScoreInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_all_user_score_info(pool: &SqlitePool, data: &EvilCastleRogueLikeAllUserScoreInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeAllUserScoreInfo (
    Uid,
    AllUserTotalScore
) VALUES (
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.all_user_total_score)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_all_user_score_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<EvilCastleRogueLikeAllUserScoreInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeAllUserScoreInfo>("SELECT * FROM EvilCastleRogueLikeAllUserScoreInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EvilCastleRogueLikeAllUserScoreInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_all_user_score_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeAllUserScoreInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}