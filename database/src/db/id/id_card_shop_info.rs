use crate::models::game::id::id_card_shop_info::IdCardShopInfo;
use sqlx::SqlitePool;

/// Add a single IdCardShopInfo record from a Rust struct.
pub async fn add_id_card_shop_info(pool: &SqlitePool, data: &IdCardShopInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO IdCardShopInfo (
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
pub async fn get_id_card_shop_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<IdCardShopInfo>> {
    sqlx::query_as::<_, IdCardShopInfo>("SELECT * FROM IdCardShopInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the account's real buy-count for one shop item, if any purchase has been made.
pub async fn get_by_uid_and_id(
    pool: &SqlitePool,
    uid: i64,
    id: i32,
) -> sqlx::Result<Option<IdCardShopInfo>> {
    sqlx::query_as::<_, IdCardShopInfo>("SELECT * FROM IdCardShopInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Bump (or create) the account's buy-count for one shop item.
pub async fn increment_buy_count(pool: &SqlitePool, uid: i64, id: i32, by: i32) -> sqlx::Result<()> {
    if get_by_uid_and_id(pool, uid, id).await?.is_some() {
        sqlx::query("UPDATE IdCardShopInfo SET BuyCount = COALESCE(BuyCount, 0) + ? WHERE Uid = ? AND Id = ?")
            .bind(by)
            .bind(uid)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO IdCardShopInfo (Uid, Id, BuyCount) VALUES (?, ?, ?)")
            .bind(uid)
            .bind(id)
            .bind(by)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Delete all IdCardShopInfo rows for a UID.
pub async fn delete_id_card_shop_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IdCardShopInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<IdCardShopInfo> {
    sqlx::query_as::<_, IdCardShopInfo>("SELECT * FROM IdCardShopInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<IdCardShopInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM IdCardShopInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, IdCardShopInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &IdCardShopInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO IdCardShopInfo (
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
