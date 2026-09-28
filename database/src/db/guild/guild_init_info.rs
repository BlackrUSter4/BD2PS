use crate::models::game::guild::guild_init_info::GuildInitInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of GuildInitInfo records for a UID.
pub async fn insert_guild_init_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("guildInitInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_guild_init_info: missing or invalid 'guildInitInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested UserBaseInfo - extract InvenIndex values
        let join_recv_info_index =
            if let Some(nested_arr) = entry.get("joinRecvInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested GuildActionInfo - extract InvenIndex values
        let action_info_index =
            if let Some(nested_arr) = entry.get("actionInfo").and_then(|v| v.as_array()) {
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
        let is_reward = entry
            .get("isReward")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        // Handle single nested GuildRaidPlayInfo - extract InvenIndex
        let raid_play_info_index = entry
            .get("raidPlayInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO GuildInitInfo (
    Uid,
    JoinRecvInfoIndex,
    ActionInfoIndex,
    IsReward,
    RaidPlayInfoIndex
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
        .bind(&join_recv_info_index)
        .bind(&action_info_index)
        .bind(&is_reward)
        .bind(&raid_play_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single GuildInitInfo record from a Rust struct.
pub async fn add_guild_init_info(pool: &SqlitePool, data: &GuildInitInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildInitInfo (
    Uid,
    JoinRecvInfoIndex,
    ActionInfoIndex,
    IsReward,
    RaidPlayInfoIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.join_recv_info_index)
    .bind(&data.action_info_index)
    .bind(&data.is_reward)
    .bind(&data.raid_play_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_init_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GuildInitInfo>> {
    sqlx::query_as::<_, GuildInitInfo>("SELECT * FROM GuildInitInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildInitInfo rows for a UID.
pub async fn delete_guild_init_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildInitInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
