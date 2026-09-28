use crate::models::game::gacha::gacha_selection_count_change_info::GachaSelectionCountChangeInfo;
use sqlx::SqlitePool;

/// Add a single GachaSelectionCountChangeInfo record from a Rust struct.
pub async fn add_gacha_selection_count_change_info(
    pool: &SqlitePool,
    data: &GachaSelectionCountChangeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaSelectionCountChangeInfo (
    Uid,
    GroupId,
    Count
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_selection_count_change_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GachaSelectionCountChangeInfo>> {
    sqlx::query_as::<_, GachaSelectionCountChangeInfo>(
        "SELECT * FROM GachaSelectionCountChangeInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all GachaSelectionCountChangeInfo rows for a UID.
pub async fn delete_gacha_selection_count_change_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaSelectionCountChangeInfo WHERE Uid = ?")
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
) -> sqlx::Result<GachaSelectionCountChangeInfo> {
    sqlx::query_as::<_, GachaSelectionCountChangeInfo>(
        "SELECT * FROM GachaSelectionCountChangeInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GachaSelectionCountChangeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GachaSelectionCountChangeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GachaSelectionCountChangeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GachaSelectionCountChangeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GachaSelectionCountChangeInfo (
    Uid,
    GroupId,
    Count
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
