use crate::models::game::mini::mini_game_run_info::MiniGameRunInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of MiniGameRunInfo records for a UID.
pub async fn insert_mini_game_run_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("miniGameRunInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_mini_game_run_info: missing or invalid 'miniGameRunInfo' array");
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
INSERT INTO MiniGameRunInfo (
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

/// Add a single MiniGameRunInfo record from a Rust struct.
pub async fn add_mini_game_run_info(pool: &SqlitePool, data: &MiniGameRunInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameRunInfo (
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
pub async fn get_mini_game_run_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameRunInfo>> {
    sqlx::query_as::<_, MiniGameRunInfo>("SELECT * FROM MiniGameRunInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameRunInfo rows for a UID.
pub async fn set_info_index(pool: &SqlitePool, uid: i64, info_index: &str) -> sqlx::Result<()> {
    let existing = get_mini_game_run_info(pool, uid).await?;
    if let Some(row) = existing.into_iter().next() {
        sqlx::query("UPDATE MiniGameRunInfo SET InfoIndex = ? WHERE \"Index\" = ?")
            .bind(info_index)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_mini_game_run_info(pool, &MiniGameRunInfo { index: 0, uid, info_index: Some(info_index.to_string()) }).await?;
    }
    Ok(())
}

pub async fn delete_mini_game_run_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameRunInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
