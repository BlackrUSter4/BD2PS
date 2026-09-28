use crate::models::game::monster::monster_hunt_rank_user_info::MonsterHuntRankUserInfo;
use sqlx::SqlitePool;

/// Add a single MonsterHuntRankUserInfo record from a Rust struct.
pub async fn add_monster_hunt_rank_user_info(
    pool: &SqlitePool,
    data: &MonsterHuntRankUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterHuntRankUserInfo (
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
    RankTopPercent
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
    .bind(&data.rank_top_percent)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_hunt_rank_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterHuntRankUserInfo>> {
    sqlx::query_as::<_, MonsterHuntRankUserInfo>(
        "SELECT * FROM MonsterHuntRankUserInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MonsterHuntRankUserInfo rows for a UID.
pub async fn delete_monster_hunt_rank_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterHuntRankUserInfo WHERE Uid = ?")
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
) -> sqlx::Result<MonsterHuntRankUserInfo> {
    sqlx::query_as::<_, MonsterHuntRankUserInfo>(
        "SELECT * FROM MonsterHuntRankUserInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MonsterHuntRankUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterHuntRankUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterHuntRankUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterHuntRankUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterHuntRankUserInfo (
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
    RankTopPercent
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
    .bind(&data.rank_top_percent)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
