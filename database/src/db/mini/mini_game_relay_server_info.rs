use crate::models::game::mini::mini_game_relay_server_info::MiniGameRelayServerInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of MiniGameRelayServerInfo records for a UID.
pub async fn insert_mini_game_relay_server_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data
        .get("miniGameRelayServerInfo")
        .and_then(|v| v.as_array())
    {
        Some(a) => a,
        None => {
            eprintln!("insert_mini_game_relay_server_info: missing or invalid 'miniGameRelayServerInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
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
INSERT INTO MiniGameRelayServerInfo (
    Uid,
    ServerInfoIndex
) VALUES (
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&server_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single MiniGameRelayServerInfo record from a Rust struct.
pub async fn add_mini_game_relay_server_info(
    pool: &SqlitePool,
    data: &MiniGameRelayServerInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameRelayServerInfo (
    Uid,
    ServerInfoIndex
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.server_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_relay_server_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameRelayServerInfo>> {
    sqlx::query_as::<_, MiniGameRelayServerInfo>(
        "SELECT * FROM MiniGameRelayServerInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameRelayServerInfo rows for a UID.
pub async fn delete_mini_game_relay_server_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameRelayServerInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
