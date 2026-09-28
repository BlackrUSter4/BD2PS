use crate::models::game::pack::pack_preview_info::PackPreviewInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of PackPreviewInfo records for a UID.
pub async fn insert_pack_preview_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("packPreviewInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_pack_preview_info: missing or invalid 'packPreviewInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested QuestInfo - extract InvenIndex values
        let quest_info_index =
            if let Some(nested_arr) = entry.get("questInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested QuestTitleInfo - extract InvenIndex values
        let quest_title_info_index =
            if let Some(nested_arr) = entry.get("questTitleInfo").and_then(|v| v.as_array()) {
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
        let is_pack_event_reward = entry
            .get("isPackEventReward")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO PackPreviewInfo (
    Uid,
    QuestInfoIndex,
    QuestTitleInfoIndex,
    IsPackEventReward
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&quest_info_index)
        .bind(&quest_title_info_index)
        .bind(&is_pack_event_reward)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single PackPreviewInfo record from a Rust struct.
pub async fn add_pack_preview_info(pool: &SqlitePool, data: &PackPreviewInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackPreviewInfo (
    Uid,
    QuestInfoIndex,
    QuestTitleInfoIndex,
    IsPackEventReward
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.quest_info_index)
    .bind(&data.quest_title_info_index)
    .bind(&data.is_pack_event_reward)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_preview_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PackPreviewInfo>> {
    sqlx::query_as::<_, PackPreviewInfo>("SELECT * FROM PackPreviewInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PackPreviewInfo rows for a UID.
pub async fn delete_pack_preview_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackPreviewInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
