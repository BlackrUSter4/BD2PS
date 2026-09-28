use crate::models::game::evil::evil_castle_reward_info::EvilCastleRewardInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of EvilCastleRewardInfo records for a UID.
pub async fn insert_evil_castle_reward_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("evilCastleRewardInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "insert_evil_castle_reward_info: missing or invalid 'evilCastleRewardInfo' array"
            );
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested EvilCastleEndSeasonInfo - extract InvenIndex values
        let end_season_info_index =
            if let Some(nested_arr) = entry.get("endSeasonInfo").and_then(|v| v.as_array()) {
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
        // Handle single nested EvilCastleEndSeasonTotalInfo - extract InvenIndex
        let end_season_total_info_index = entry
            .get("endSeasonTotalInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());
        // Handle single nested RewardInfoBundle - extract InvenIndex
        let reward_info_bundle_index = entry
            .get("rewardInfoBundle")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO EvilCastleRewardInfo (
    Uid,
    EndSeasonInfoIndex,
    EndSeasonTotalInfoIndex,
    RewardInfoBundleIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&end_season_info_index)
        .bind(&end_season_total_info_index)
        .bind(&reward_info_bundle_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EvilCastleRewardInfo record from a Rust struct.
pub async fn add_evil_castle_reward_info(
    pool: &SqlitePool,
    data: &EvilCastleRewardInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRewardInfo (
    Uid,
    EndSeasonInfoIndex,
    EndSeasonTotalInfoIndex,
    RewardInfoBundleIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.end_season_info_index)
    .bind(&data.end_season_total_info_index)
    .bind(&data.reward_info_bundle_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_reward_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRewardInfo>> {
    sqlx::query_as::<_, EvilCastleRewardInfo>("SELECT * FROM EvilCastleRewardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EvilCastleRewardInfo rows for a UID.
pub async fn delete_evil_castle_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRewardInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
