use crate::models::game::id::id_card_item_info::IdCardItemInfo;
use sqlx::SqlitePool;

/// Add a single IdCardItemInfo record from a Rust struct.
pub async fn add_id_card_item_info(pool: &SqlitePool, data: &IdCardItemInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO IdCardItemInfo (
    Uid,
    InvenIndex,
    Id,
    X,
    Y,
    Rotate,
    Scale,
    Layer,
    Color
) VALUES (
    ?,
    ?,
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
    .bind(&data.id)
    .bind(&data.x)
    .bind(&data.y)
    .bind(&data.rotate)
    .bind(&data.scale)
    .bind(&data.layer)
    .bind(&data.color)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_id_card_item_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<IdCardItemInfo>> {
    sqlx::query_as::<_, IdCardItemInfo>("SELECT * FROM IdCardItemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all IdCardItemInfo rows for a UID.
pub async fn delete_id_card_item_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IdCardItemInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<IdCardItemInfo> {
    sqlx::query_as::<_, IdCardItemInfo>("SELECT * FROM IdCardItemInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<IdCardItemInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM IdCardItemInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, IdCardItemInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &IdCardItemInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO IdCardItemInfo (
    Uid,
    InvenIndex,
    Id,
    X,
    Y,
    Rotate,
    Scale,
    Layer,
    Color
) VALUES (
    ?,
    ?,
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
    .bind(&data.id)
    .bind(&data.x)
    .bind(&data.y)
    .bind(&data.rotate)
    .bind(&data.scale)
    .bind(&data.layer)
    .bind(&data.color)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
