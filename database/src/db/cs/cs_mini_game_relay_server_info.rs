use crate::models::game::cs::cs_mini_game_relay_server_info::CSMiniGameRelayServerInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of CSMiniGameRelayServerInfo records for a UID.
pub async fn insert_cs_mini_game_relay_server_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data
        .get("cSMiniGameRelayServerInfo")
        .and_then(|v| v.as_array())
    {
        Some(a) => a,
        None => {
            eprintln!("insert_cs_mini_game_relay_server_info: missing or invalid 'cSMiniGameRelayServerInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle single nested CommonHeader - extract InvenIndex
        let header_index = entry
            .get("header")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());
        // Handle repeated nested RelayServerInfo - extract InvenIndex values
        let server_info_index =
            if let Some(nested_arr) = entry.get("serverInfo").and_then(|v| v.as_array()) {
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
INSERT INTO CSMiniGameRelayServerInfo (
    Uid,
    HeaderIndex,
    ServerInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&header_index)
        .bind(&server_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single CSMiniGameRelayServerInfo record from a Rust struct.
pub async fn add_cs_mini_game_relay_server_info(
    pool: &SqlitePool,
    data: &CSMiniGameRelayServerInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CSMiniGameRelayServerInfo (
    Uid,
    HeaderIndex,
    ServerInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.header_index)
    .bind(&data.server_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cs_mini_game_relay_server_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CSMiniGameRelayServerInfo>> {
    sqlx::query_as::<_, CSMiniGameRelayServerInfo>(
        "SELECT * FROM CSMiniGameRelayServerInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all CSMiniGameRelayServerInfo rows for a UID.
pub async fn delete_cs_mini_game_relay_server_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CSMiniGameRelayServerInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
