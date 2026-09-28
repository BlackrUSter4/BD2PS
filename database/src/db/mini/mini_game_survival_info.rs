use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::mini::mini_game_survival_info::MiniGameSurvivalInfo;
/// Insert a full JSON array of MiniGameSurvivalInfo records for a UID.
pub async fn insert_mini_game_survival_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("miniGameSurvivalInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_mini_game_survival_info: missing or invalid 'miniGameSurvivalInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle parallel repeated arrays - iterate all together
        let event_schedule_id = entry
            .get("eventScheduleId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let user_rank_score = entry
            .get("userRankScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let top_rank_owner_index = entry
            .get("topRankOwnerIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let top_rank_user_id = entry
            .get("topRankUserId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let top_rank_score = entry
            .get("topRankScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let stage_clear_info_index = entry
            .get("stageClearInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let collection_info_index = entry
            .get("collectionInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let upgrade_info_index = entry
            .get("upgradeInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Get all parallel arrays
        let active_char_id_array = entry.get("activeCharId").and_then(|v| v.as_array());
        let active_map_group_id_array = entry.get("activeMapGroupId").and_then(|v| v.as_array());

        // Determine max length
        let len = 0
            .max(active_char_id_array.map(|a| a.len()).unwrap_or(0))
            .max(active_map_group_id_array.map(|a| a.len()).unwrap_or(0));

        if len > 0 {
            // Insert one row per index (parallel iteration)
            for i in 0..len {
                let active_char_id = active_char_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let active_map_group_id = active_map_group_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;

                sqlx::query(
                    r#"
INSERT INTO MiniGameSurvivalInfo (
    Uid,
    EventScheduleId,
    ActiveCharId,
    ActiveMapGroupId,
    UserRankScore,
    TopRankOwnerIndex,
    TopRankUserId,
    TopRankScore,
    StageClearInfoIndex,
    CollectionInfoIndex,
    UpgradeInfoIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&event_schedule_id)
                .bind(active_char_id)
                .bind(active_map_group_id)
                .bind(&user_rank_score)
                .bind(&top_rank_owner_index)
                .bind(&top_rank_user_id)
                .bind(&top_rank_score)
                .bind(&stage_clear_info_index)
                .bind(&collection_info_index)
                .bind(&upgrade_info_index)
                .execute(pool)
                .await?;
            }
        } else {
            // No items, insert one row with NULLs for repeated fields
            sqlx::query(
                r#"
INSERT INTO MiniGameSurvivalInfo (
    Uid,
    EventScheduleId,
    ActiveCharId,
    ActiveMapGroupId,
    UserRankScore,
    TopRankOwnerIndex,
    TopRankUserId,
    TopRankScore,
    StageClearInfoIndex,
    CollectionInfoIndex,
    UpgradeInfoIndex
) VALUES (
    ?,
    ?,
    NULL,
    NULL,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
            )
            .bind(uid)
            .bind(&event_schedule_id)
            .bind(&user_rank_score)
            .bind(&top_rank_owner_index)
            .bind(&top_rank_user_id)
            .bind(&top_rank_score)
            .bind(&stage_clear_info_index)
            .bind(&collection_info_index)
            .bind(&upgrade_info_index)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Add a single MiniGameSurvivalInfo record from a Rust struct.
pub async fn add_mini_game_survival_info(pool: &SqlitePool, data: &MiniGameSurvivalInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSurvivalInfo (
    Uid,
    EventScheduleId,
    ActiveCharId,
    ActiveMapGroupId,
    UserRankScore,
    TopRankOwnerIndex,
    TopRankUserId,
    TopRankScore,
    StageClearInfoIndex,
    CollectionInfoIndex,
    UpgradeInfoIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.event_schedule_id)
    .bind(&data.active_char_id)
    .bind(&data.active_map_group_id)
    .bind(&data.user_rank_score)
    .bind(&data.top_rank_owner_index)
    .bind(&data.top_rank_user_id)
    .bind(&data.top_rank_score)
    .bind(&data.stage_clear_info_index)
    .bind(&data.collection_info_index)
    .bind(&data.upgrade_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_survival_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MiniGameSurvivalInfo>> {
    sqlx::query_as::<_, MiniGameSurvivalInfo>("SELECT * FROM MiniGameSurvivalInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameSurvivalInfo rows for a UID.
pub async fn delete_mini_game_survival_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSurvivalInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}