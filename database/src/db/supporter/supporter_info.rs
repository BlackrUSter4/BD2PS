use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::supporter::supporter_info::SupporterInfo;
/// Insert SupporterInfo data containing multiple sub-arrays for a UID.
pub async fn insert_supporter_info(_pool: &SqlitePool, data: &Value, _uid: i64) -> sqlx::Result<()> {
    // --- CostumeBaseInfo ---
    if let Some(arr) = data.get("supporterInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for CostumeBaseInfo and insert
            // This needs to be customized based on the actual fields in CostumeBaseInfo
        }
    }
    // --- CostumeBaseInfo ---
    if let Some(arr) = data.get("guildSupporterInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for CostumeBaseInfo and insert
            // This needs to be customized based on the actual fields in CostumeBaseInfo
        }
    }
    Ok(())
}

/// Add a single SupporterInfo record from a Rust struct.
pub async fn add_supporter_info(pool: &SqlitePool, data: &SupporterInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SupporterInfo (
    Uid,
    SupporterInfoIndex,
    GuildSupporterInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.supporter_info_index)
    .bind(&data.guild_supporter_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_supporter_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<SupporterInfo>> {
    sqlx::query_as::<_, SupporterInfo>("SELECT * FROM SupporterInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all SupporterInfo rows for a UID.
pub async fn delete_supporter_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SupporterInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}