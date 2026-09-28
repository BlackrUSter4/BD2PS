use crate::models::game::costume::costume_info::CostumeInfo;
use sqlx::SqlitePool;

/// Add a single CostumeInfo record from a Rust struct.
pub async fn add_costume_info(pool: &SqlitePool, data: &CostumeInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CostumeInfo (
    Uid,
    InvenIndex,
    Id,
    Level,
    UseChar,
    PictorialbookInfoIndex,
    SortId,
    UseMyRoomCount,
    DesignId
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
    .bind(&data.level)
    .bind(&data.use_char)
    .bind(&data.pictorialbook_info_index)
    .bind(&data.sort_id)
    .bind(&data.use_my_room_count)
    .bind(&data.design_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_costume_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CostumeInfo>> {
    sqlx::query_as::<_, CostumeInfo>("SELECT * FROM CostumeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CostumeInfo rows for a UID.
pub async fn delete_costume_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CostumeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<CostumeInfo> {
    sqlx::query_as::<_, CostumeInfo>("SELECT * FROM CostumeInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<CostumeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CostumeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CostumeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CostumeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CostumeInfo (
    Uid,
    InvenIndex,
    Id,
    Level,
    UseChar,
    PictorialbookInfoIndex,
    SortId,
    UseMyRoomCount,
    DesignId
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
    .bind(&data.level)
    .bind(&data.use_char)
    .bind(&data.pictorialbook_info_index)
    .bind(&data.sort_id)
    .bind(&data.use_my_room_count)
    .bind(&data.design_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn set_level(pool: &SqlitePool, uid: i64, index: i64, level: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE CostumeInfo SET Level = ? WHERE Uid = ? AND Index = ?")
        .bind(level)
        .bind(uid)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
) -> sqlx::Result<Option<CostumeInfo>> {
    sqlx::query_as::<_, CostumeInfo>("SELECT * FROM CostumeInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .fetch_optional(pool)
        .await
}

pub async fn set_use_char(
    pool: &SqlitePool,
    uid: i64,
    costume_inven_index: i64,
    char_inven_index: i64,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE CostumeInfo SET UseChar = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(char_inven_index)
        .bind(uid)
        .bind(costume_inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_level_by_inven(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    level: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE CostumeInfo SET Level = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(level)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}
