use crate::models::game::deck::deck_info::DeckInfo;
use sqlx::SqlitePool;

/// Add a single DeckInfo record from a Rust struct.
pub async fn add_deck_info(pool: &SqlitePool, data: &DeckInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DeckInfo (
    Uid,
    CharInvenIndex,
    Position,
    Sequence
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
    .bind(&data.position)
    .bind(&data.sequence)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<DeckInfo>> {
    sqlx::query_as::<_, DeckInfo>("SELECT * FROM DeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all DeckInfo rows for a UID.
pub async fn delete_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Replace the account's whole active deck (delete + reinsert), inside a transaction.
pub async fn replace_deck_info(
    pool: &SqlitePool,
    uid: i64,
    entries: &[(i64, Option<i32>, Option<i32>)],
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM DeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    for (char_inven_index, position, sequence) in entries {
        sqlx::query(
            "INSERT INTO DeckInfo (Uid, CharInvenIndex, Position, Sequence) VALUES (?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(char_inven_index)
        .bind(position)
        .bind(sequence)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<DeckInfo> {
    sqlx::query_as::<_, DeckInfo>("SELECT * FROM DeckInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<DeckInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DeckInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DeckInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DeckInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DeckInfo (
    Uid,
    CharInvenIndex,
    Position,
    Sequence
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
    .bind(&data.position)
    .bind(&data.sequence)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
