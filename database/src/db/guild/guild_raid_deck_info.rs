use crate::models::game::guild::guild_raid_deck_info::GuildRaidDeckInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of GuildRaidDeckInfo records for a UID.
pub async fn insert_guild_raid_deck_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("guildRaidDeckInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_guild_raid_deck_info: missing or invalid 'guildRaidDeckInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested DeckInfo - extract InvenIndex values
        let deck_info_index =
            if let Some(nested_arr) = entry.get("deckInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested GuildRaidSupporterDeckInfo - extract InvenIndex values
        let supporter_deck_info_index =
            if let Some(nested_arr) = entry.get("supporterDeckInfo").and_then(|v| v.as_array()) {
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
        let is_supporter_deck_update = entry
            .get("isSupporterDeckUpdate")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO GuildRaidDeckInfo (
    Uid,
    DeckInfoIndex,
    SupporterDeckInfoIndex,
    IsSupporterDeckUpdate
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&deck_info_index)
        .bind(&supporter_deck_info_index)
        .bind(&is_supporter_deck_update)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single GuildRaidDeckInfo record from a Rust struct.
pub async fn add_guild_raid_deck_info(
    pool: &SqlitePool,
    data: &GuildRaidDeckInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidDeckInfo (
    Uid,
    DeckInfoIndex,
    SupporterDeckInfoIndex,
    IsSupporterDeckUpdate
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.deck_info_index)
    .bind(&data.supporter_deck_info_index)
    .bind(&data.is_supporter_deck_update)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_deck_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidDeckInfo>> {
    sqlx::query_as::<_, GuildRaidDeckInfo>("SELECT * FROM GuildRaidDeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildRaidDeckInfo rows for a UID.
pub async fn delete_guild_raid_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidDeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
