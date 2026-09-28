use crate::models::game::field::field_char_control_deck_type::FieldCharControlDeckType;
use sqlx::SqlitePool;

/// Add a single FieldCharControlDeckType record from a Rust struct.
pub async fn add_field_char_control_deck_type(
    pool: &SqlitePool,
    data: &FieldCharControlDeckType,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FieldCharControlDeckType (
    Uid,
    FieldCharControlDeckType
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.field_char_control_deck_type)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_field_char_control_deck_type(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<FieldCharControlDeckType>> {
    sqlx::query_as::<_, FieldCharControlDeckType>(
        "SELECT * FROM FieldCharControlDeckType WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all FieldCharControlDeckType rows for a UID.
pub async fn set(pool: &SqlitePool, uid: i64, value: i32) -> sqlx::Result<()> {
    let existing = get_field_char_control_deck_type(pool, uid).await?;
    if let Some(row) = existing.into_iter().next() {
        sqlx::query("UPDATE FieldCharControlDeckType SET FieldCharControlDeckType = ? WHERE \"Index\" = ?")
            .bind(value)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_field_char_control_deck_type(pool, &crate::models::game::field::field_char_control_deck_type::FieldCharControlDeckType { index: 0, uid, field_char_control_deck_type: Some(value) }).await?;
    }
    Ok(())
}

pub async fn delete_field_char_control_deck_type(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FieldCharControlDeckType WHERE Uid = ?")
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
) -> sqlx::Result<FieldCharControlDeckType> {
    sqlx::query_as::<_, FieldCharControlDeckType>(
        "SELECT * FROM FieldCharControlDeckType WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<FieldCharControlDeckType>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM FieldCharControlDeckType WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, FieldCharControlDeckType>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &FieldCharControlDeckType) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO FieldCharControlDeckType (
    Uid,
    FieldCharControlDeckType
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.field_char_control_deck_type)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
