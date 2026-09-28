use crate::models::game::mini::mini_game_rhythm_info::MiniGameRhythmInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of MiniGameRhythmInfo records for a UID.
pub async fn insert_mini_game_rhythm_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("miniGameRhythmInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "insert_mini_game_rhythm_info: missing or invalid 'miniGameRhythmInfo' array"
            );
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested MiniGameRhythmPlayInfo - extract InvenIndex values
        let play_info_index =
            if let Some(nested_arr) = entry.get("playInfo").and_then(|v| v.as_array()) {
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
        // Handle single nested RewardInfoBundle - extract InvenIndex
        let reward_info_bundle_index = entry
            .get("rewardInfoBundle")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO MiniGameRhythmInfo (
    Uid,
    PlayInfoIndex,
    RewardInfoBundleIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&play_info_index)
        .bind(&reward_info_bundle_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single MiniGameRhythmInfo record from a Rust struct.
pub async fn add_mini_game_rhythm_info(
    pool: &SqlitePool,
    data: &MiniGameRhythmInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameRhythmInfo (
    Uid,
    PlayInfoIndex,
    RewardInfoBundleIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.play_info_index)
    .bind(&data.reward_info_bundle_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_rhythm_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameRhythmInfo>> {
    sqlx::query_as::<_, MiniGameRhythmInfo>("SELECT * FROM MiniGameRhythmInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameRhythmInfo rows for a UID.
pub async fn delete_mini_game_rhythm_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameRhythmInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
