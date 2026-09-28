use crate::models::game::item::item_auto_upgrade_info::ItemAutoUpgradeInfo;
use sqlx::SqlitePool;

/// Add a single ItemAutoUpgradeInfo record from a Rust struct.
pub async fn add_item_auto_upgrade_info(
    pool: &SqlitePool,
    data: &ItemAutoUpgradeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ItemAutoUpgradeInfo (
    Uid,
    InvenIndex,
    ItemType,
    ItemId,
    BeforeLevel,
    AfterLevel,
    SortId
) VALUES (
    ?,
    ?,
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
    .bind(&data.item_type)
    .bind(&data.item_id)
    .bind(&data.before_level)
    .bind(&data.after_level)
    .bind(&data.sort_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_item_auto_upgrade_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ItemAutoUpgradeInfo>> {
    sqlx::query_as::<_, ItemAutoUpgradeInfo>("SELECT * FROM ItemAutoUpgradeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ItemAutoUpgradeInfo rows for a UID.
pub async fn delete_item_auto_upgrade_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ItemAutoUpgradeInfo WHERE Uid = ?")
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
) -> sqlx::Result<ItemAutoUpgradeInfo> {
    sqlx::query_as::<_, ItemAutoUpgradeInfo>(
        "SELECT * FROM ItemAutoUpgradeInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<ItemAutoUpgradeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ItemAutoUpgradeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ItemAutoUpgradeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ItemAutoUpgradeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ItemAutoUpgradeInfo (
    Uid,
    InvenIndex,
    ItemType,
    ItemId,
    BeforeLevel,
    AfterLevel,
    SortId
) VALUES (
    ?,
    ?,
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
    .bind(&data.item_type)
    .bind(&data.item_id)
    .bind(&data.before_level)
    .bind(&data.after_level)
    .bind(&data.sort_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
