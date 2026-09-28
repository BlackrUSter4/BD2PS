use crate::models::game::battle::battle_result_info::BattleResultInfo;
use sqlx::SqlitePool;

/// Add a single BattleResultInfo record from a Rust struct.
pub async fn add_battle_result_info(
    pool: &SqlitePool,
    data: &BattleResultInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO BattleResultInfo (
    Uid,
    BattleResult,
    RedCharInfoIndex,
    BlueCharInfoIndex,
    GridItemIndex,
    BattleStatisticsInfoIndex,
    GolemInfoIndex,
    RedDeckOutCharInfoIndex,
    BlueDeckOutCharInfoIndex
) VALUES (
    ?,
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
    .bind(&data.battle_result)
    .bind(&data.red_char_info_index)
    .bind(&data.blue_char_info_index)
    .bind(&data.grid_item_index)
    .bind(&data.battle_statistics_info_index)
    .bind(&data.golem_info_index)
    .bind(&data.red_deck_out_char_info_index)
    .bind(&data.blue_deck_out_char_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_battle_result_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<BattleResultInfo>> {
    sqlx::query_as::<_, BattleResultInfo>("SELECT * FROM BattleResultInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all BattleResultInfo rows for a UID.
pub async fn delete_battle_result_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM BattleResultInfo WHERE Uid = ?")
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
) -> sqlx::Result<BattleResultInfo> {
    sqlx::query_as::<_, BattleResultInfo>(
        "SELECT * FROM BattleResultInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<BattleResultInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM BattleResultInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, BattleResultInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &BattleResultInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO BattleResultInfo (
    Uid,
    BattleResult,
    RedCharInfoIndex,
    BlueCharInfoIndex,
    GridItemIndex,
    BattleStatisticsInfoIndex,
    GolemInfoIndex,
    RedDeckOutCharInfoIndex,
    BlueDeckOutCharInfoIndex
) VALUES (
    ?,
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
    .bind(&data.battle_result)
    .bind(&data.red_char_info_index)
    .bind(&data.blue_char_info_index)
    .bind(&data.grid_item_index)
    .bind(&data.battle_statistics_info_index)
    .bind(&data.golem_info_index)
    .bind(&data.red_deck_out_char_info_index)
    .bind(&data.blue_deck_out_char_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
