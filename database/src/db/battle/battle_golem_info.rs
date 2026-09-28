use crate::models::game::battle::battle_golem_info::BattleGolemInfo;
use sqlx::SqlitePool;

/// Add a single BattleGolemInfo record from a Rust struct.
pub async fn add_battle_golem_info(pool: &SqlitePool, data: &BattleGolemInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO BattleGolemInfo (
    Uid,
    Level,
    Gauge,
    RemainTurn,
    ReserveCostumeId,
    Key,
    Value
) VALUES (
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
    .bind(&data.level)
    .bind(&data.gauge)
    .bind(&data.remain_turn)
    .bind(&data.reserve_costume_id)
    .bind(&data.key)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_battle_golem_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<BattleGolemInfo>> {
    sqlx::query_as::<_, BattleGolemInfo>("SELECT * FROM BattleGolemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all BattleGolemInfo rows for a UID.
pub async fn delete_battle_golem_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM BattleGolemInfo WHERE Uid = ?")
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
) -> sqlx::Result<BattleGolemInfo> {
    sqlx::query_as::<_, BattleGolemInfo>(
        "SELECT * FROM BattleGolemInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<BattleGolemInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM BattleGolemInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, BattleGolemInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &BattleGolemInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO BattleGolemInfo (
    Uid,
    Level,
    Gauge,
    RemainTurn,
    ReserveCostumeId,
    Key,
    Value
) VALUES (
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
    .bind(&data.level)
    .bind(&data.gauge)
    .bind(&data.remain_turn)
    .bind(&data.reserve_costume_id)
    .bind(&data.key)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
