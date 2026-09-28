use crate::models::game::evil::evil_castle_ranking_info::EvilCastleRankingInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of EvilCastleRankingInfo records for a UID.
pub async fn insert_evil_castle_ranking_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("evilCastleRankingInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "insert_evil_castle_ranking_info: missing or invalid 'evilCastleRankingInfo' array"
            );
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested EvilCastleRankUserInfo - extract InvenIndex values
        let user_ranking_info_index =
            if let Some(nested_arr) = entry.get("userRankingInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle single nested EvilCastleRankUserInfo - extract InvenIndex
        let my_ranking_info_index = entry
            .get("myRankingInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO EvilCastleRankingInfo (
    Uid,
    UserRankingInfoIndex,
    MyRankingInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&user_ranking_info_index)
        .bind(&my_ranking_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EvilCastleRankingInfo record from a Rust struct.
pub async fn add_evil_castle_ranking_info(
    pool: &SqlitePool,
    data: &EvilCastleRankingInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRankingInfo (
    Uid,
    UserRankingInfoIndex,
    MyRankingInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.user_ranking_info_index)
    .bind(&data.my_ranking_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_ranking_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRankingInfo>> {
    sqlx::query_as::<_, EvilCastleRankingInfo>("SELECT * FROM EvilCastleRankingInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EvilCastleRankingInfo rows for a UID.
pub async fn delete_evil_castle_ranking_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRankingInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
