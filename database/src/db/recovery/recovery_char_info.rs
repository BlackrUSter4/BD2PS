use crate::models::game::recovery::recovery_char_info::RecoveryCharInfo;
use sqlx::SqlitePool;

/// Add a single RecoveryCharInfo record from a Rust struct.
pub async fn add_recovery_char_info(
    pool: &SqlitePool,
    data: &RecoveryCharInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO RecoveryCharInfo (
    Uid,
    CharInvenIndex,
    Hp
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.char_inven_index)
    .bind(&data.hp)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_recovery_char_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<RecoveryCharInfo>> {
    sqlx::query_as::<_, RecoveryCharInfo>("SELECT * FROM RecoveryCharInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all RecoveryCharInfo rows for a UID.
pub async fn delete_recovery_char_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM RecoveryCharInfo WHERE Uid = ?")
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
) -> sqlx::Result<RecoveryCharInfo> {
    sqlx::query_as::<_, RecoveryCharInfo>(
        "SELECT * FROM RecoveryCharInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<RecoveryCharInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM RecoveryCharInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, RecoveryCharInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &RecoveryCharInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO RecoveryCharInfo (
    Uid,
    CharInvenIndex,
    Hp
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.char_inven_index)
    .bind(&data.hp)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
