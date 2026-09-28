use crate::models::game::guild::guild_raid_preset_info::GuildRaidPresetInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of GuildRaidPresetInfo records for a UID.
pub async fn insert_guild_raid_preset_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("guildRaidPresetInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "insert_guild_raid_preset_info: missing or invalid 'guildRaidPresetInfo' array"
            );
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested PresetInfo - extract InvenIndex values
        let preset_info_index =
            if let Some(nested_arr) = entry.get("presetInfo").and_then(|v| v.as_array()) {
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

        sqlx::query(
            r#"
INSERT INTO GuildRaidPresetInfo (
    Uid,
    PresetInfoIndex
) VALUES (
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&preset_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single GuildRaidPresetInfo record from a Rust struct.
pub async fn add_guild_raid_preset_info(
    pool: &SqlitePool,
    data: &GuildRaidPresetInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidPresetInfo (
    Uid,
    PresetInfoIndex
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.preset_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_preset_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidPresetInfo>> {
    sqlx::query_as::<_, GuildRaidPresetInfo>("SELECT * FROM GuildRaidPresetInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildRaidPresetInfo rows for a UID.
pub async fn delete_guild_raid_preset_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidPresetInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
