use crate::models::game::total::total_war_info::TotalWarInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of TotalWarInfo records for a UID.
pub async fn insert_total_war_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("totalWarInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_total_war_info: missing or invalid 'totalWarInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested BattleDamageInfo - extract InvenIndex values
        let score_info_index =
            if let Some(nested_arr) = entry.get("scoreInfo").and_then(|v| v.as_array()) {
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
        let top_percent = entry
            .get("topPercent")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let top_ranker_score = entry
            .get("topRankerScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let engine_type = entry
            .get("engineType")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        sqlx::query(
            r#"
INSERT INTO TotalWarInfo (
    Uid,
    ScoreInfoIndex,
    TopPercent,
    TopRankerScore,
    EngineType
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&score_info_index)
        .bind(&top_percent)
        .bind(&top_ranker_score)
        .bind(&engine_type)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single TotalWarInfo record from a Rust struct.
pub async fn add_total_war_info(pool: &SqlitePool, data: &TotalWarInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TotalWarInfo (
    Uid,
    ScoreInfoIndex,
    TopPercent,
    TopRankerScore,
    EngineType,
    ClaimedRewardIds
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
    .bind(&data.score_info_index)
    .bind(&data.top_percent)
    .bind(&data.top_ranker_score)
    .bind(&data.engine_type)
    .bind(&data.claimed_reward_ids)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_total_war_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<TotalWarInfo>> {
    sqlx::query_as::<_, TotalWarInfo>("SELECT * FROM TotalWarInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the account's single TotalWarInfo row, creating a fresh one if it doesn't exist yet.
pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<TotalWarInfo> {
    if let Some(row) = get_total_war_info(pool, uid).await?.into_iter().next() {
        return Ok(row);
    }
    let fresh = TotalWarInfo {
        index: 0,
        uid,
        score_info_index: None,
        top_percent: Some(0.0),
        top_ranker_score: Some(0),
        engine_type: None,
        claimed_reward_ids: None,
    };
    add_total_war_info(pool, &fresh).await?;
    Ok(get_total_war_info(pool, uid)
        .await?
        .into_iter()
        .next()
        .expect("row was just inserted"))
}

/// Full-row update, matched by Uid (there is exactly one TotalWarInfo row per account).
pub async fn update_total_war_info(pool: &SqlitePool, data: &TotalWarInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
UPDATE TotalWarInfo SET
    ScoreInfoIndex = ?,
    TopPercent = ?,
    TopRankerScore = ?,
    EngineType = ?,
    ClaimedRewardIds = ?
WHERE Uid = ?
"#,
    )
    .bind(&data.score_info_index)
    .bind(&data.top_percent)
    .bind(&data.top_ranker_score)
    .bind(&data.engine_type)
    .bind(&data.claimed_reward_ids)
    .bind(&data.uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Real cross-account ranking by cumulative score (parsed from ScoreInfoIndex's stored JSON
/// sum, computed in Rust since it's stored as a JSON blob, not a column).
pub async fn all_rows(pool: &SqlitePool) -> sqlx::Result<Vec<TotalWarInfo>> {
    sqlx::query_as::<_, TotalWarInfo>("SELECT * FROM TotalWarInfo").fetch_all(pool).await
}

/// Delete all TotalWarInfo rows for a UID.
pub async fn delete_total_war_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TotalWarInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
