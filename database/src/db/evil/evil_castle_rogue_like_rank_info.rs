use crate::models::game::evil::evil_castle_rogue_like_rank_info::EvilCastleRogueLikeRankInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of EvilCastleRogueLikeRankInfo records for a UID.
pub async fn insert_evil_castle_rogue_like_rank_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data
        .get("evilCastleRogueLikeRankInfo")
        .and_then(|v| v.as_array())
    {
        Some(a) => a,
        None => {
            eprintln!("insert_evil_castle_rogue_like_rank_info: missing or invalid 'evilCastleRogueLikeRankInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested EvilCastleRogueLikeRankUserInfo - extract InvenIndex values
        let user_rank_info_index =
            if let Some(nested_arr) = entry.get("userRankInfo").and_then(|v| v.as_array()) {
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
        // Handle single nested EvilCastleRogueLikeRankUserInfo - extract InvenIndex
        let my_rank_info_index = entry
            .get("myRankInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO EvilCastleRogueLikeRankInfo (
    Uid,
    UserRankInfoIndex,
    MyRankInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&user_rank_info_index)
        .bind(&my_rank_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EvilCastleRogueLikeRankInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_rank_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeRankInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeRankInfo (
    Uid,
    UserRankInfoIndex,
    MyRankInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.user_rank_info_index)
    .bind(&data.my_rank_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeRankInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeRankInfo>(
        "SELECT * FROM EvilCastleRogueLikeRankInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeRankInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeRankInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
