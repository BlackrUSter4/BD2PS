use crate::models::game::total::total_war_deck_info::TotalWarDeckInfo;
use sqlx::SqlitePool;

/// Add a single TotalWarDeckInfo record from a Rust struct.
pub async fn add_total_war_deck_info(
    pool: &SqlitePool,
    data: &TotalWarDeckInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TotalWarDeckInfo (
    Uid,
    PlayType,
    InvenIndex,
    CharInvenIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.play_type)
    .bind(&data.inven_index)
    .bind(&data.char_inven_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_total_war_deck_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<TotalWarDeckInfo>> {
    sqlx::query_as::<_, TotalWarDeckInfo>("SELECT * FROM TotalWarDeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Full replace: delete the account's current deck rows and insert the new list. Matches
/// TotalWarDeckSaveRequest's shape (the client always sends the whole deck, not a delta).
pub async fn replace_all(
    pool: &SqlitePool,
    uid: i64,
    items: &[TotalWarDeckInfo],
) -> sqlx::Result<()> {
    delete_total_war_deck_info(pool, uid).await?;
    for item in items {
        sqlx::query(
            "INSERT INTO TotalWarDeckInfo (Uid, PlayType, InvenIndex, CharInvenIndex) VALUES (?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(&item.play_type)
        .bind(&item.inven_index)
        .bind(&item.char_inven_index)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Delete all TotalWarDeckInfo rows for a UID.
pub async fn delete_total_war_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TotalWarDeckInfo WHERE Uid = ?")
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
) -> sqlx::Result<TotalWarDeckInfo> {
    sqlx::query_as::<_, TotalWarDeckInfo>(
        "SELECT * FROM TotalWarDeckInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<TotalWarDeckInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM TotalWarDeckInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, TotalWarDeckInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &TotalWarDeckInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO TotalWarDeckInfo (
    Uid,
    PlayType,
    InvenIndex,
    CharInvenIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.play_type)
    .bind(&data.inven_index)
    .bind(&data.char_inven_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
