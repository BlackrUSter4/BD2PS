use crate::models::game::cash::cash_shop_purchase_count_info::CashShopPurchaseCountInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of CashShopPurchaseCountInfo records for a UID.
pub async fn insert_cash_shop_purchase_count_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data
        .get("cashShopPurchaseCountInfo")
        .and_then(|v| v.as_array())
    {
        Some(a) => a,
        None => {
            eprintln!("insert_cash_shop_purchase_count_info: missing or invalid 'cashShopPurchaseCountInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested PurchaseCountInfo - extract InvenIndex values
        let purchase_count_info_index =
            if let Some(nested_arr) = entry.get("purchaseCountInfo").and_then(|v| v.as_array()) {
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
INSERT INTO CashShopPurchaseCountInfo (
    Uid,
    PurchaseCountInfoIndex
) VALUES (
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&purchase_count_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single CashShopPurchaseCountInfo record from a Rust struct.
pub async fn add_cash_shop_purchase_count_info(
    pool: &SqlitePool,
    data: &CashShopPurchaseCountInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CashShopPurchaseCountInfo (
    Uid,
    PurchaseCountInfoIndex
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.purchase_count_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cash_shop_purchase_count_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CashShopPurchaseCountInfo>> {
    sqlx::query_as::<_, CashShopPurchaseCountInfo>(
        "SELECT * FROM CashShopPurchaseCountInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all CashShopPurchaseCountInfo rows for a UID.
pub async fn delete_cash_shop_purchase_count_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CashShopPurchaseCountInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
