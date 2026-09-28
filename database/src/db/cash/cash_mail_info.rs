use crate::models::game::cash::cash_mail_info::CashMailInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of CashMailInfo records for a UID.
pub async fn insert_cash_mail_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("cashMailInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_cash_mail_info: missing or invalid 'cashMailInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested MailInfo - extract InvenIndex values
        let mail_info_index =
            if let Some(nested_arr) = entry.get("mailInfo").and_then(|v| v.as_array()) {
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
        let total_count = entry
            .get("totalCount")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let max_inven_index = entry
            .get("maxInvenIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        sqlx::query(
            r#"
INSERT INTO CashMailInfo (
    Uid,
    MailInfoIndex,
    TotalCount,
    MaxInvenIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&mail_info_index)
        .bind(&total_count)
        .bind(&max_inven_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single CashMailInfo record from a Rust struct.
pub async fn add_cash_mail_info(pool: &SqlitePool, data: &CashMailInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CashMailInfo (
    Uid,
    MailInfoIndex,
    TotalCount,
    MaxInvenIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.mail_info_index)
    .bind(&data.total_count)
    .bind(&data.max_inven_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cash_mail_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CashMailInfo>> {
    sqlx::query_as::<_, CashMailInfo>("SELECT * FROM CashMailInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CashMailInfo rows for a UID.
pub async fn delete_cash_mail_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CashMailInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
