use crate::models::game::contents::contents_char_item_info::ContentsCharItemInfo;
use sqlx::SqlitePool;

/// Add a single ContentsCharItemInfo record from a Rust struct.
pub async fn add_contents_char_item_info(
    pool: &SqlitePool,
    data: &ContentsCharItemInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ContentsCharItemInfo (
    Uid,
    CharInvenIndex,
    EquipInfoIndex,
    ConnectPotentialCostume
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.char_inven_index)
    .bind(&data.equip_info_index)
    .bind(&data.connect_potential_costume)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_contents_char_item_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ContentsCharItemInfo>> {
    sqlx::query_as::<_, ContentsCharItemInfo>("SELECT * FROM ContentsCharItemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ContentsCharItemInfo rows for a UID.
pub async fn delete_contents_char_item_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ContentsCharItemInfo WHERE Uid = ?")
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
) -> sqlx::Result<ContentsCharItemInfo> {
    sqlx::query_as::<_, ContentsCharItemInfo>(
        "SELECT * FROM ContentsCharItemInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<ContentsCharItemInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ContentsCharItemInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ContentsCharItemInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ContentsCharItemInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ContentsCharItemInfo (
    Uid,
    CharInvenIndex,
    EquipInfoIndex,
    ConnectPotentialCostume
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.char_inven_index)
    .bind(&data.equip_info_index)
    .bind(&data.connect_potential_costume)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
