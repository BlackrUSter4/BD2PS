use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::supporter::supporter_battle_info::SupporterBattleInfo;
/// Insert SupporterBattleInfo data containing multiple sub-arrays for a UID.
pub async fn insert_supporter_battle_info(_pool: &SqlitePool, data: &Value, _uid: i64) -> sqlx::Result<()> {
    // --- SupporterDeckInfo ---
    if let Some(arr) = data.get("friendSupporterInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for SupporterDeckInfo and insert
            // This needs to be customized based on the actual fields in SupporterDeckInfo
        }
    }
    // --- SupporterDeckInfo ---
    if let Some(arr) = data.get("recommendedSupporterInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for SupporterDeckInfo and insert
            // This needs to be customized based on the actual fields in SupporterDeckInfo
        }
    }
    Ok(())
}

/// Add a single SupporterBattleInfo record from a Rust struct.
pub async fn add_supporter_battle_info(pool: &SqlitePool, data: &SupporterBattleInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SupporterBattleInfo (
    Uid,
    FriendSupporterInfoIndex,
    RecommendedSupporterInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.friend_supporter_info_index)
    .bind(&data.recommended_supporter_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_supporter_battle_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<SupporterBattleInfo>> {
    sqlx::query_as::<_, SupporterBattleInfo>("SELECT * FROM SupporterBattleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all SupporterBattleInfo rows for a UID.
pub async fn delete_supporter_battle_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SupporterBattleInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}