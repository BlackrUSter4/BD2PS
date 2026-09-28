use crate::models::game::gacha::gacha_selection_info::GachaSelectionInfo;
use sqlx::SqlitePool;

/// Add a single GachaSelectionInfo record from a Rust struct.
pub async fn add_gacha_selection_info(
    pool: &SqlitePool,
    data: &GachaSelectionInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaSelectionInfo (
    Uid,
    GroupId,
    Slot,
    ItemId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.slot)
    .bind(&data.item_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_selection_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GachaSelectionInfo>> {
    sqlx::query_as::<_, GachaSelectionInfo>("SELECT * FROM GachaSelectionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GachaSelectionInfo rows for a UID.
pub async fn delete_gacha_selection_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaSelectionInfo WHERE Uid = ?")
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
) -> sqlx::Result<GachaSelectionInfo> {
    sqlx::query_as::<_, GachaSelectionInfo>(
        "SELECT * FROM GachaSelectionInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GachaSelectionInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GachaSelectionInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GachaSelectionInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GachaSelectionInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GachaSelectionInfo (
    Uid,
    GroupId,
    Slot,
    ItemId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.slot)
    .bind(&data.item_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
