use crate::models::game::mini::mini_game_field_info::MiniGameFieldInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of MiniGameFieldInfo records for a UID.
pub async fn insert_mini_game_field_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("miniGameFieldInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_mini_game_field_info: missing or invalid 'miniGameFieldInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested MiniGameInfo - extract InvenIndex values
        let info_index = if let Some(nested_arr) = entry.get("info").and_then(|v| v.as_array()) {
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
INSERT INTO MiniGameFieldInfo (
    Uid,
    InfoIndex
) VALUES (
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single MiniGameFieldInfo record from a Rust struct.
pub async fn add_mini_game_field_info(
    pool: &SqlitePool,
    data: &MiniGameFieldInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameFieldInfo (
    Uid,
    InfoIndex
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_field_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameFieldInfo>> {
    sqlx::query_as::<_, MiniGameFieldInfo>("SELECT * FROM MiniGameFieldInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameFieldInfo rows for a UID.
pub async fn delete_mini_game_field_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameFieldInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
