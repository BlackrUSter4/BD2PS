use crate::models::game::popular::popular_costume_info::PopularCostumeInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of PopularCostumeInfo records for a UID.
pub async fn insert_popular_costume_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("popularCostumeInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_popular_costume_info: missing or invalid 'popularCostumeInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested PopularCostumeCountInfo - extract InvenIndex values
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
INSERT INTO PopularCostumeInfo (
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

/// Add a single PopularCostumeInfo record from a Rust struct.
pub async fn add_popular_costume_info(
    pool: &SqlitePool,
    data: &PopularCostumeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PopularCostumeInfo (
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
pub async fn get_popular_costume_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PopularCostumeInfo>> {
    sqlx::query_as::<_, PopularCostumeInfo>("SELECT * FROM PopularCostumeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PopularCostumeInfo rows for a UID.
pub async fn delete_popular_costume_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PopularCostumeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
