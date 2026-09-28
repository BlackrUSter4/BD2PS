use crate::models::game::pvp::pvp_battle_rank_detail_info::PvpBattleRankDetailInfo;
use sqlx::SqlitePool;

/// Add a single PvpBattleRankDetailInfo record from a Rust struct.
pub async fn add_pvp_battle_rank_detail_info(
    pool: &SqlitePool,
    data: &PvpBattleRankDetailInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleRankDetailInfo (
    Uid,
    OwnerIndex,
    Vp,
    Rank,
    WinCount,
    LoseCount,
    PortraitCostumeId,
    DeckInfo
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
    .bind(&data.owner_index)
    .bind(&data.vp)
    .bind(&data.rank)
    .bind(&data.win_count)
    .bind(&data.lose_count)
    .bind(&data.portrait_costume_id)
    .bind(&data.deck_info)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_rank_detail_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PvpBattleRankDetailInfo>> {
    sqlx::query_as::<_, PvpBattleRankDetailInfo>(
        "SELECT * FROM PvpBattleRankDetailInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all PvpBattleRankDetailInfo rows for a UID.
pub async fn delete_pvp_battle_rank_detail_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleRankDetailInfo WHERE Uid = ?")
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
) -> sqlx::Result<PvpBattleRankDetailInfo> {
    sqlx::query_as::<_, PvpBattleRankDetailInfo>(
        "SELECT * FROM PvpBattleRankDetailInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PvpBattleRankDetailInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PvpBattleRankDetailInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PvpBattleRankDetailInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PvpBattleRankDetailInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PvpBattleRankDetailInfo (
    Uid,
    OwnerIndex,
    Vp,
    Rank,
    WinCount,
    LoseCount,
    PortraitCostumeId,
    DeckInfo
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
    .bind(&data.owner_index)
    .bind(&data.vp)
    .bind(&data.rank)
    .bind(&data.win_count)
    .bind(&data.lose_count)
    .bind(&data.portrait_costume_id)
    .bind(&data.deck_info)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
