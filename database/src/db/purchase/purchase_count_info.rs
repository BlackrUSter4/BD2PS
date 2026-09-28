use crate::models::game::purchase::purchase_count_info::PurchaseCountInfo;
use sqlx::SqlitePool;

/// Add a single PurchaseCountInfo record from a Rust struct.
pub async fn add_purchase_count_info(
    pool: &SqlitePool,
    data: &PurchaseCountInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PurchaseCountInfo (
    Uid,
    GroupId,
    Id,
    SaleGroup,
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.sale_group)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_purchase_count_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PurchaseCountInfo>> {
    sqlx::query_as::<_, PurchaseCountInfo>("SELECT * FROM PurchaseCountInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_product(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    id: i32,
    sale_group: i32,
) -> sqlx::Result<Option<PurchaseCountInfo>> {
    sqlx::query_as::<_, PurchaseCountInfo>(
        "SELECT * FROM PurchaseCountInfo WHERE Uid = ? AND GroupId = ? AND Id = ? AND SaleGroup = ?",
    )
    .bind(uid)
    .bind(group_id)
    .bind(id)
    .bind(sale_group)
    .fetch_optional(pool)
    .await
}

pub async fn increment(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    id: i32,
    sale_group: i32,
) -> sqlx::Result<i32> {
    if let Some(row) = get_by_product(pool, uid, group_id, id, sale_group).await? {
        let new_count = row.count.unwrap_or(0) + 1;
        sqlx::query("UPDATE PurchaseCountInfo SET Count = ? WHERE \"Index\" = ?")
            .bind(new_count)
            .bind(row.index)
            .execute(pool)
            .await?;
        Ok(new_count)
    } else {
        add_purchase_count_info(
            pool,
            &PurchaseCountInfo { index: 0, uid, group_id: Some(group_id), id: Some(id), sale_group: Some(sale_group), count: Some(1) },
        )
        .await?;
        Ok(1)
    }
}

/// Delete all PurchaseCountInfo rows for a UID.
pub async fn delete_purchase_count_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PurchaseCountInfo WHERE Uid = ?")
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
) -> sqlx::Result<PurchaseCountInfo> {
    sqlx::query_as::<_, PurchaseCountInfo>(
        "SELECT * FROM PurchaseCountInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PurchaseCountInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PurchaseCountInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PurchaseCountInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PurchaseCountInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PurchaseCountInfo (
    Uid,
    GroupId,
    Id,
    SaleGroup,
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.sale_group)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
