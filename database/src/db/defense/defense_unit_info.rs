use crate::models::game::defense::defense_unit_info::DefenseUnitInfo;
use sqlx::SqlitePool;

/// Add a single DefenseUnitInfo record from a Rust struct.
pub async fn add_defense_unit_info(pool: &SqlitePool, data: &DefenseUnitInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DefenseUnitInfo (
    Uid,
    UnitId,
    UnitIndex,
    SpawnTicks,
    DespwanTicks,
    ElementLevel,
    CoolTime,
    GridIndex
) VALUES (
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
    .bind(&data.unit_id)
    .bind(&data.unit_index)
    .bind(&data.spawn_ticks)
    .bind(&data.despwan_ticks)
    .bind(&data.element_level)
    .bind(&data.cool_time)
    .bind(&data.grid_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_defense_unit_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DefenseUnitInfo>> {
    sqlx::query_as::<_, DefenseUnitInfo>("SELECT * FROM DefenseUnitInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all DefenseUnitInfo rows for a UID.
pub async fn delete_defense_unit_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DefenseUnitInfo WHERE Uid = ?")
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
) -> sqlx::Result<DefenseUnitInfo> {
    sqlx::query_as::<_, DefenseUnitInfo>(
        "SELECT * FROM DefenseUnitInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<DefenseUnitInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DefenseUnitInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DefenseUnitInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DefenseUnitInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DefenseUnitInfo (
    Uid,
    UnitId,
    UnitIndex,
    SpawnTicks,
    DespwanTicks,
    ElementLevel,
    CoolTime,
    GridIndex
) VALUES (
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
    .bind(&data.unit_id)
    .bind(&data.unit_index)
    .bind(&data.spawn_ticks)
    .bind(&data.despwan_ticks)
    .bind(&data.element_level)
    .bind(&data.cool_time)
    .bind(&data.grid_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
