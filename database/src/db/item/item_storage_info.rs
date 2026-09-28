use crate::models::game::item::item_storage_info::ItemStorageInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of ItemStorageInfo records for a UID.
pub async fn insert_item_storage_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("itemStorageInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_item_storage_info: missing or invalid 'itemStorageInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested ItemInfo - extract InvenIndex values
        let item_info_index =
            if let Some(nested_arr) = entry.get("itemInfo").and_then(|v| v.as_array()) {
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
INSERT INTO ItemStorageInfo (
    Uid,
    ItemInfoIndex
) VALUES (
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&item_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single ItemStorageInfo record from a Rust struct.
pub async fn add_item_storage_info(pool: &SqlitePool, data: &ItemStorageInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ItemStorageInfo (
    Uid,
    ItemInfoIndex
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.item_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_item_storage_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ItemStorageInfo>> {
    sqlx::query_as::<_, ItemStorageInfo>("SELECT * FROM ItemStorageInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ItemStorageInfo rows for a UID.
pub async fn delete_item_storage_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ItemStorageInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
