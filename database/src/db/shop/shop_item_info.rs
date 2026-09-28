use crate::models::game::shop::shop_item_info::ShopItemInfo;
use sqlx::SqlitePool;

/// Add a single ShopItemInfo record from a Rust struct.
pub async fn add_shop_item_info(pool: &SqlitePool, data: &ShopItemInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ShopItemInfo (
    Uid,
    InvenIndex,
    Id,
    ItemCount,
    Rate
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
    .bind(&data.item_count)
    .bind(&data.rate)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_shop_item_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ShopItemInfo>> {
    sqlx::query_as::<_, ShopItemInfo>("SELECT * FROM ShopItemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ShopItemInfo rows for a UID.
pub async fn delete_shop_item_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ShopItemInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ShopItemInfo> {
    sqlx::query_as::<_, ShopItemInfo>("SELECT * FROM ShopItemInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ShopItemInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ShopItemInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ShopItemInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ShopItemInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ShopItemInfo (
    Uid,
    InvenIndex,
    Id,
    ItemCount,
    Rate
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
    .bind(&data.item_count)
    .bind(&data.rate)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
