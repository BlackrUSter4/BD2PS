use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::mini::mini_game_defense_info::MiniGameDefenseInfo;
/// Insert a full JSON array of MiniGameDefenseInfo records for a UID.
pub async fn insert_mini_game_defense_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("miniGameDefenseInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_mini_game_defense_info: missing or invalid 'miniGameDefenseInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated primitive fields - insert one row per value
        let reward_info_array = entry.get("rewardInfo").and_then(|v| v.as_array());
        if let Some(values) = reward_info_array {
            for item in values {
                let event_schedule_id = entry
                    .get("eventScheduleId")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let reward_info = item.as_i64().unwrap_or_default() as i32;

                sqlx::query(
                    r#"
INSERT INTO MiniGameDefenseInfo (
    Uid,
    EventScheduleId,
    RewardInfo
) VALUES (
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&event_schedule_id)
                .bind(&reward_info)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Add a single MiniGameDefenseInfo record from a Rust struct.
pub async fn add_mini_game_defense_info(pool: &SqlitePool, data: &MiniGameDefenseInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameDefenseInfo (
    Uid,
    EventScheduleId,
    RewardInfo
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.event_schedule_id)
    .bind(&data.reward_info)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_defense_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MiniGameDefenseInfo>> {
    sqlx::query_as::<_, MiniGameDefenseInfo>("SELECT * FROM MiniGameDefenseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameDefenseInfo rows for a UID.
pub async fn delete_mini_game_defense_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameDefenseInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}