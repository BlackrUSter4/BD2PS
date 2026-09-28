use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::mini::mini_game_action_info::MiniGameActionInfo;
/// Insert a full JSON array of MiniGameActionInfo records for a UID.
pub async fn insert_mini_game_action_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("miniGameActionInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_mini_game_action_info: missing or invalid 'miniGameActionInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated primitive fields - insert one row per value
        let clear_mission_id_array = entry.get("clearMissionId").and_then(|v| v.as_array());
        if let Some(values) = clear_mission_id_array {
            for item in values {
                let event_schedule_id = entry
                    .get("eventScheduleId")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let my_best_record_index = entry
                    .get("myBestRecordIndex")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let clear_mission_id = item.as_i64().unwrap_or_default() as i32;

                sqlx::query(
                    r#"
INSERT INTO MiniGameActionInfo (
    Uid,
    EventScheduleId,
    MyBestRecordIndex,
    ClearMissionId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&event_schedule_id)
                .bind(&my_best_record_index)
                .bind(&clear_mission_id)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Add a single MiniGameActionInfo record from a Rust struct.
pub async fn add_mini_game_action_info(pool: &SqlitePool, data: &MiniGameActionInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameActionInfo (
    Uid,
    EventScheduleId,
    MyBestRecordIndex,
    ClearMissionId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.event_schedule_id)
    .bind(&data.my_best_record_index)
    .bind(&data.clear_mission_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_action_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MiniGameActionInfo>> {
    sqlx::query_as::<_, MiniGameActionInfo>("SELECT * FROM MiniGameActionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameActionInfo rows for a UID.
pub async fn delete_mini_game_action_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameActionInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}