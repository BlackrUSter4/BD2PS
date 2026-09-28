use crate::models::game::gacha::gacha_info::GachaInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of GachaInfo records for a UID.
pub async fn insert_gacha_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("gachaInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_gacha_info: missing or invalid 'gachaInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested GachaScheduleInfo - extract InvenIndex values
        let schedule_info_index =
            if let Some(nested_arr) = entry.get("scheduleInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested GachaUserInfo - extract InvenIndex values
        let gacha_user_info_index =
            if let Some(nested_arr) = entry.get("gachaUserInfo").and_then(|v| v.as_array()) {
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
        let schedule_end_exchange_point = entry
            .get("scheduleEndExchangePoint")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        // Handle repeated nested GachaFixedInfo - extract InvenIndex values
        let gacha_fixed_info_index =
            if let Some(nested_arr) = entry.get("gachaFixedInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested GachaSelectionInfo - extract InvenIndex values
        let gacha_selection_info_index =
            if let Some(nested_arr) = entry.get("gachaSelectionInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested GachaSelectionCountChangeInfo - extract InvenIndex values
        let gacha_selection_change_count_info_index = if let Some(nested_arr) = entry
            .get("gachaSelectionChangeCountInfo")
            .and_then(|v| v.as_array())
        {
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
        // Handle repeated nested GachaStepUpScheduleInfo - extract InvenIndex values
        let step_up_schedule_info_index =
            if let Some(nested_arr) = entry.get("stepUpScheduleInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested GachaStepUpUserInfo - extract InvenIndex values
        let step_up_user_info_index =
            if let Some(nested_arr) = entry.get("stepUpUserInfo").and_then(|v| v.as_array()) {
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
        // Handle repeated nested ResemaraGachaInfo - extract InvenIndex values
        let resemara_preview_item_info_index = if let Some(nested_arr) = entry
            .get("resemaraPreviewItemInfo")
            .and_then(|v| v.as_array())
        {
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
INSERT INTO GachaInfo (
    Uid,
    ScheduleInfoIndex,
    GachaUserInfoIndex,
    ScheduleEndExchangePoint,
    GachaFixedInfoIndex,
    GachaSelectionInfoIndex,
    GachaSelectionChangeCountInfoIndex,
    StepUpScheduleInfoIndex,
    StepUpUserInfoIndex,
    ResemaraPreviewItemInfoIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&schedule_info_index)
        .bind(&gacha_user_info_index)
        .bind(&schedule_end_exchange_point)
        .bind(&gacha_fixed_info_index)
        .bind(&gacha_selection_info_index)
        .bind(&gacha_selection_change_count_info_index)
        .bind(&step_up_schedule_info_index)
        .bind(&step_up_user_info_index)
        .bind(&resemara_preview_item_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single GachaInfo record from a Rust struct.
pub async fn add_gacha_info(pool: &SqlitePool, data: &GachaInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaInfo (
    Uid,
    ScheduleInfoIndex,
    GachaUserInfoIndex,
    ScheduleEndExchangePoint,
    GachaFixedInfoIndex,
    GachaSelectionInfoIndex,
    GachaSelectionChangeCountInfoIndex,
    StepUpScheduleInfoIndex,
    StepUpUserInfoIndex,
    ResemaraPreviewItemInfoIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.schedule_info_index)
    .bind(&data.gacha_user_info_index)
    .bind(&data.schedule_end_exchange_point)
    .bind(&data.gacha_fixed_info_index)
    .bind(&data.gacha_selection_info_index)
    .bind(&data.gacha_selection_change_count_info_index)
    .bind(&data.step_up_schedule_info_index)
    .bind(&data.step_up_user_info_index)
    .bind(&data.resemara_preview_item_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GachaInfo>> {
    sqlx::query_as::<_, GachaInfo>("SELECT * FROM GachaInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GachaInfo rows for a UID.
pub async fn delete_gacha_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
