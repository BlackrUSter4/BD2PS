use crate::models::game::pvp::pvp_battle_rank_user_info::PvpBattleRankUserInfo;
use sqlx::SqlitePool;

/// Add a single PvpBattleRankUserInfo record from a Rust struct.
pub async fn add_pvp_battle_rank_user_info(
    pool: &SqlitePool,
    data: &PvpBattleRankUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleRankUserInfo (
    Uid,
    OwnerIndex,
    UserId,
    UserExp,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    GuildBaseInfoIndex,
    BaseInfo,
    TitleId
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
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.user_exp)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.guild_base_info_index)
    .bind(&data.base_info_index)
    .bind(&data.title_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_rank_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PvpBattleRankUserInfo>> {
    sqlx::query_as::<_, PvpBattleRankUserInfo>("SELECT * FROM PvpBattleRankUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PvpBattleRankUserInfo rows for a UID.
pub async fn delete_pvp_battle_rank_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleRankUserInfo WHERE Uid = ?")
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
) -> sqlx::Result<PvpBattleRankUserInfo> {
    sqlx::query_as::<_, PvpBattleRankUserInfo>(
        "SELECT * FROM PvpBattleRankUserInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PvpBattleRankUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PvpBattleRankUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PvpBattleRankUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PvpBattleRankUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PvpBattleRankUserInfo (
    Uid,
    OwnerIndex,
    UserId,
    UserExp,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    GuildBaseInfoIndex,
    BaseInfo,
    TitleId
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
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.user_exp)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.guild_base_info_index)
    .bind(&data.base_info_index)
    .bind(&data.title_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
