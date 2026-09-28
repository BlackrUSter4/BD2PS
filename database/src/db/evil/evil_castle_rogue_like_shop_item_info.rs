use crate::models::game::evil::evil_castle_rogue_like_shop_item_info::EvilCastleRogueLikeShopItemInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeShopItemInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_shop_item_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeShopItemInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeShopItemInfo (
    Uid,
    Type,
    Id,
    Price,
    SoldOut
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
    .bind(&data.r#type)
    .bind(&data.id)
    .bind(&data.price)
    .bind(&data.sold_out)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_shop_item_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeShopItemInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeShopItemInfo>(
        "SELECT * FROM EvilCastleRogueLikeShopItemInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeShopItemInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_shop_item_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeShopItemInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeShopItemInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeShopItemInfo>(
        "SELECT * FROM EvilCastleRogueLikeShopItemInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeShopItemInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeShopItemInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeShopItemInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeShopItemInfo,
) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeShopItemInfo (
    Uid,
    Type,
    Id,
    Price,
    SoldOut
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
    .bind(&data.r#type)
    .bind(&data.id)
    .bind(&data.price)
    .bind(&data.sold_out)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Replace the caller's whole shop item list (delete-then-insert-all).
pub async fn save_all(
    pool: &SqlitePool,
    uid: i64,
    items: &[EvilCastleRogueLikeShopItemInfo],
) -> sqlx::Result<()> {
    delete_evil_castle_rogue_like_shop_item_info(pool, uid).await?;
    for item in items {
        insert(pool, item).await?;
    }
    Ok(())
}
