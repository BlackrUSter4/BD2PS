use crate::models::game::pvp::pvp_battle_user_info::PvpBattleUserInfo;
use sqlx::SqlitePool;

/// Add a single PvpBattleUserInfo record from a Rust struct.
pub async fn add_pvp_battle_user_info(
    pool: &SqlitePool,
    data: &PvpBattleUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleUserInfo (
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
pub async fn get_pvp_battle_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PvpBattleUserInfo>> {
    sqlx::query_as::<_, PvpBattleUserInfo>("SELECT * FROM PvpBattleUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PvpBattleUserInfo rows for a UID.
pub async fn delete_pvp_battle_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleUserInfo WHERE Uid = ?")
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
) -> sqlx::Result<PvpBattleUserInfo> {
    sqlx::query_as::<_, PvpBattleUserInfo>(
        "SELECT * FROM PvpBattleUserInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PvpBattleUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PvpBattleUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PvpBattleUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PvpBattleUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PvpBattleUserInfo (
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
