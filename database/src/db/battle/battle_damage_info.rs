use crate::models::game::battle::battle_damage_info::BattleDamageInfo;
use sqlx::SqlitePool;

/// Add a single BattleDamageInfo record from a Rust struct.
pub async fn add_battle_damage_info(
    pool: &SqlitePool,
    data: &BattleDamageInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO BattleDamageInfo (
    Uid,
    Id,
    Value
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_battle_damage_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<BattleDamageInfo>> {
    sqlx::query_as::<_, BattleDamageInfo>("SELECT * FROM BattleDamageInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all BattleDamageInfo rows for a UID.
pub async fn delete_battle_damage_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM BattleDamageInfo WHERE Uid = ?")
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
) -> sqlx::Result<BattleDamageInfo> {
    sqlx::query_as::<_, BattleDamageInfo>(
        "SELECT * FROM BattleDamageInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<BattleDamageInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM BattleDamageInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, BattleDamageInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &BattleDamageInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO BattleDamageInfo (
    Uid,
    Id,
    Value
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
