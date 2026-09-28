use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::guild::guild_join_send_info::GuildJoinSendInfo;
/// Insert GuildJoinSendInfo data containing multiple sub-arrays for a UID.
pub async fn insert_guild_join_send_info(_pool: &SqlitePool, data: &Value, _uid: i64) -> sqlx::Result<()> {
    // --- GuildInfo ---
    if let Some(arr) = data.get("joinSendInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for GuildInfo and insert
            // This needs to be customized based on the actual fields in GuildInfo
        }
    }
    // --- GuildActionInfo ---
    if let Some(arr) = data.get("actionInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for GuildActionInfo and insert
            // This needs to be customized based on the actual fields in GuildActionInfo
        }
    }
    Ok(())
}

/// Add a single GuildJoinSendInfo record from a Rust struct.
pub async fn add_guild_join_send_info(pool: &SqlitePool, data: &GuildJoinSendInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildJoinSendInfo (
    Uid,
    JoinSendInfoIndex,
    ActionInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.join_send_info_index)
    .bind(&data.action_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_join_send_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GuildJoinSendInfo>> {
    sqlx::query_as::<_, GuildJoinSendInfo>("SELECT * FROM GuildJoinSendInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildJoinSendInfo rows for a UID.
pub async fn delete_guild_join_send_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildJoinSendInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}