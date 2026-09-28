use crate::models::game::evil::evil_castle_rogue_like_rank_user_info::EvilCastleRogueLikeRankUserInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeRankUserInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_rank_user_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeRankUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeRankUserInfo (
    Uid,
    OwnerIndex,
    UserId,
    UserExp,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    GuildBaseInfoIndex,
    Rank,
    Score,
    TitleId,
    CrystalDamage
) VALUES (
    ?,
    ?,
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
    .bind(&data.rank)
    .bind(&data.score)
    .bind(&data.title_id)
    .bind(&data.crystal_damage)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_rank_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeRankUserInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeRankUserInfo>(
        "SELECT * FROM EvilCastleRogueLikeRankUserInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeRankUserInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_rank_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeRankUserInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeRankUserInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeRankUserInfo>(
        "SELECT * FROM EvilCastleRogueLikeRankUserInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeRankUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeRankUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeRankUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeRankUserInfo,
) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeRankUserInfo (
    Uid,
    OwnerIndex,
    UserId,
    UserExp,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    GuildBaseInfoIndex,
    Rank,
    Score,
    TitleId,
    CrystalDamage
) VALUES (
    ?,
    ?,
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
    .bind(&data.rank)
    .bind(&data.score)
    .bind(&data.title_id)
    .bind(&data.crystal_damage)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
