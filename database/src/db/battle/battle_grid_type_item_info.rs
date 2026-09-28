use crate::models::game::battle::battle_grid_type_item_info::BattleGridTypeItemInfo;
use sqlx::SqlitePool;

/// Add a single BattleGridTypeItemInfo record from a Rust struct.
pub async fn add_battle_grid_type_item_info(
    pool: &SqlitePool,
    data: &BattleGridTypeItemInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO BattleGridTypeItemInfo (
    Uid,
    TeamType,
    GridIndex,
    BuffId,
    IsInfinite,
    BuffTurn
) VALUES (
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
    .bind(&data.team_type)
    .bind(&data.grid_index)
    .bind(&data.buff_id)
    .bind(&data.is_infinite)
    .bind(&data.buff_turn)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_battle_grid_type_item_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<BattleGridTypeItemInfo>> {
    sqlx::query_as::<_, BattleGridTypeItemInfo>(
        "SELECT * FROM BattleGridTypeItemInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all BattleGridTypeItemInfo rows for a UID.
pub async fn delete_battle_grid_type_item_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM BattleGridTypeItemInfo WHERE Uid = ?")
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
) -> sqlx::Result<BattleGridTypeItemInfo> {
    sqlx::query_as::<_, BattleGridTypeItemInfo>(
        "SELECT * FROM BattleGridTypeItemInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<BattleGridTypeItemInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM BattleGridTypeItemInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, BattleGridTypeItemInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &BattleGridTypeItemInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO BattleGridTypeItemInfo (
    Uid,
    TeamType,
    GridIndex,
    BuffId,
    IsInfinite,
    BuffTurn
) VALUES (
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
    .bind(&data.team_type)
    .bind(&data.grid_index)
    .bind(&data.buff_id)
    .bind(&data.is_infinite)
    .bind(&data.buff_turn)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
