use crate::models::game::item::item_material_info::ItemMaterialInfo;
use sqlx::SqlitePool;

/// Add a single ItemMaterialInfo record from a Rust struct.
pub async fn add_item_material_info(
    pool: &SqlitePool,
    data: &ItemMaterialInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ItemMaterialInfo (
    Uid,
    InvenIndex,
    Id,
    Type,
    Count
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.r#type)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_item_material_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ItemMaterialInfo>> {
    sqlx::query_as::<_, ItemMaterialInfo>("SELECT * FROM ItemMaterialInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ItemMaterialInfo rows for a UID.
pub async fn delete_item_material_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ItemMaterialInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<ItemMaterialInfo> {
    sqlx::query_as::<_, ItemMaterialInfo>(
        "SELECT * FROM ItemMaterialInfo WHERE Uid = ? AND Index = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<ItemMaterialInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ItemMaterialInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ItemMaterialInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ItemMaterialInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ItemMaterialInfo (
    Uid,
    InvenIndex,
    Id,
    Type,
    Count
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.r#type)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
