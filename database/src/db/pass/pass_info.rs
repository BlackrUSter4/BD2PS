use crate::models::game::pass::pass_info::PassInfo;
use sqlx::SqlitePool;

/// Add a single PassInfo record from a Rust struct.
pub async fn add_pass_info(pool: &SqlitePool, data: &PassInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PassInfo (
    Uid,
    Id,
    Exp,
    ActivePremium1
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.exp)
    .bind(&data.active_premium_1)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pass_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PassInfo>> {
    sqlx::query_as::<_, PassInfo>("SELECT * FROM PassInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_pass_id(pool: &SqlitePool, uid: i64, id: i32) -> sqlx::Result<Option<PassInfo>> {
    sqlx::query_as::<_, PassInfo>("SELECT * FROM PassInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn set_active_premium_1(pool: &SqlitePool, uid: i64, id: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE PassInfo SET ActivePremium1 = 1 WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all PassInfo rows for a UID.
pub async fn delete_pass_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PassInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<PassInfo> {
    sqlx::query_as::<_, PassInfo>("SELECT * FROM PassInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<PassInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PassInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PassInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PassInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PassInfo (
    Uid,
    Id,
    Exp,
    ActivePremium1
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.exp)
    .bind(&data.active_premium_1)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
