use crate::models::game::deck::deck_message_info::DeckMessageInfo;
use sqlx::SqlitePool;

/// Add a single DeckMessageInfo record from a Rust struct.
pub async fn add_deck_message_info(pool: &SqlitePool, data: &DeckMessageInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DeckMessageInfo (
    Uid,
    CharInfoIndex,
    EquipInfoIndex,
    CostumeInfoIndex,
    CharAwakeInfoIndex
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
    .bind(&data.char_info_index)
    .bind(&data.equip_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.char_awake_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_deck_message_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DeckMessageInfo>> {
    sqlx::query_as::<_, DeckMessageInfo>("SELECT * FROM DeckMessageInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all DeckMessageInfo rows for a UID.
pub async fn delete_deck_message_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DeckMessageInfo WHERE Uid = ?")
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
) -> sqlx::Result<DeckMessageInfo> {
    sqlx::query_as::<_, DeckMessageInfo>(
        "SELECT * FROM DeckMessageInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<DeckMessageInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DeckMessageInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DeckMessageInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DeckMessageInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DeckMessageInfo (
    Uid,
    CharInfoIndex,
    EquipInfoIndex,
    CostumeInfoIndex,
    CharAwakeInfoIndex
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
    .bind(&data.char_info_index)
    .bind(&data.equip_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.char_awake_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
