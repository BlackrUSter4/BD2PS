use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::dating::dating_info::DatingInfo;
/// Insert DatingInfo data containing multiple sub-arrays for a UID.
pub async fn insert_dating_info(_pool: &SqlitePool, data: &Value, _uid: i64) -> sqlx::Result<()> {
    // --- DatingEpisodeInfo ---
    if let Some(arr) = data.get("episodeInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for DatingEpisodeInfo and insert
            // This needs to be customized based on the actual fields in DatingEpisodeInfo
        }
    }
    // --- DatingMessageChoiceInfo ---
    if let Some(arr) = data.get("messageChoiceInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for DatingMessageChoiceInfo and insert
            // This needs to be customized based on the actual fields in DatingMessageChoiceInfo
        }
    }
    Ok(())
}

/// Add a single DatingInfo record from a Rust struct.
pub async fn add_dating_info(pool: &SqlitePool, data: &DatingInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DatingInfo (
    Uid,
    EpisodeInfoIndex,
    MessageChoiceInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.episode_info_index)
    .bind(&data.message_choice_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_dating_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<DatingInfo>> {
    sqlx::query_as::<_, DatingInfo>("SELECT * FROM DatingInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all DatingInfo rows for a UID.
pub async fn delete_dating_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DatingInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}