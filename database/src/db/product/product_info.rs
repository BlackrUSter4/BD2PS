use crate::models::game::product::product_info::ProductInfo;
use sqlx::SqlitePool;

/// Add a single ProductInfo record from a Rust struct.
pub async fn add_product_info(pool: &SqlitePool, data: &ProductInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ProductInfo (
    Uid,
    Id,
    BuyCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.buy_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_product_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ProductInfo>> {
    sqlx::query_as::<_, ProductInfo>("SELECT * FROM ProductInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ProductInfo rows for a UID.
pub async fn delete_product_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ProductInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ProductInfo> {
    sqlx::query_as::<_, ProductInfo>("SELECT * FROM ProductInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ProductInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ProductInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ProductInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ProductInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ProductInfo (
    Uid,
    Id,
    BuyCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.buy_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
