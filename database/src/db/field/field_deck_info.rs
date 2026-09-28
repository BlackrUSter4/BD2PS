use crate::models::game::field::field_deck_info::FieldDeckInfo;
use sqlx::SqlitePool;

/// Add a single FieldDeckInfo record from a Rust struct.
pub async fn add_field_deck_info(pool: &SqlitePool, data: &FieldDeckInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FieldDeckInfo (
    Uid,
    Sequence,
    CharInvenIndex,
    CostumeInvenIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.sequence)
    .bind(&data.char_inven_index)
    .bind(&data.costume_inven_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_field_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FieldDeckInfo>> {
    sqlx::query_as::<_, FieldDeckInfo>("SELECT * FROM FieldDeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Full replace: delete the account's current field deck and insert the new list (the
/// client always resends the whole deck, same as every other "*DeckSave" request in this
/// project).
pub async fn replace_all(pool: &SqlitePool, uid: i64, items: &[FieldDeckInfo]) -> sqlx::Result<()> {
    delete_field_deck_info(pool, uid).await?;
    for item in items {
        sqlx::query(
            "INSERT INTO FieldDeckInfo (Uid, Sequence, CharInvenIndex, CostumeInvenIndex) VALUES (?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(&item.sequence)
        .bind(&item.char_inven_index)
        .bind(&item.costume_inven_index)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Delete all FieldDeckInfo rows for a UID.
pub async fn delete_field_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FieldDeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<FieldDeckInfo> {
    sqlx::query_as::<_, FieldDeckInfo>("SELECT * FROM FieldDeckInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<FieldDeckInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM FieldDeckInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, FieldDeckInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &FieldDeckInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO FieldDeckInfo (
    Uid,
    Sequence,
    CharInvenIndex,
    CostumeInvenIndex
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.sequence)
    .bind(&data.char_inven_index)
    .bind(&data.costume_inven_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
