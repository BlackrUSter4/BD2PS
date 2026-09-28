use crate::models::game::equip::equip_storage_info::EquipStorageInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of EquipStorageInfo records for a UID.
pub async fn insert_equip_storage_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("equipStorageInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_equip_storage_info: missing or invalid 'equipStorageInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested EquipInfo - extract InvenIndex values
        let equip_info_index =
            if let Some(nested_arr) = entry.get("equipInfo").and_then(|v| v.as_array()) {
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
INSERT INTO EquipStorageInfo (
    Uid,
    EquipInfoIndex
) VALUES (
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&equip_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EquipStorageInfo record from a Rust struct.
pub async fn add_equip_storage_info(
    pool: &SqlitePool,
    data: &EquipStorageInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EquipStorageInfo (
    Uid,
    EquipInfoIndex
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.equip_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Gets the set of equip InvenIndex values currently "in storage" for a UID, stored as a single
/// JSON array in one row's EquipInfoIndex column (created lazily on first use).
pub async fn get_stored_indices(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<i64>> {
    let row = sqlx::query_as::<_, EquipStorageInfo>(
        "SELECT * FROM EquipStorageInfo WHERE Uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row
        .and_then(|r| r.equip_info_index)
        .and_then(|s| serde_json::from_str::<Vec<i64>>(&s).ok())
        .unwrap_or_default())
}

pub async fn set_stored_indices(pool: &SqlitePool, uid: i64, indices: &[i64]) -> sqlx::Result<()> {
    let json = serde_json::to_string(indices).unwrap_or_default();
    let existing = sqlx::query_as::<_, EquipStorageInfo>(
        "SELECT * FROM EquipStorageInfo WHERE Uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    if let Some(row) = existing {
        sqlx::query("UPDATE EquipStorageInfo SET EquipInfoIndex = ? WHERE \"Index\" = ?")
            .bind(&json)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO EquipStorageInfo (Uid, EquipInfoIndex) VALUES (?, ?)")
            .bind(uid)
            .bind(&json)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_equip_storage_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EquipStorageInfo>> {
    sqlx::query_as::<_, EquipStorageInfo>("SELECT * FROM EquipStorageInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EquipStorageInfo rows for a UID.
pub async fn delete_equip_storage_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EquipStorageInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
