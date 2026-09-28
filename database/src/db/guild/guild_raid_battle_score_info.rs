use crate::models::game::guild::guild_raid_battle_score_info::GuildRaidBattleScoreInfo;
use sqlx::SqlitePool;

/// Add a single GuildRaidBattleScoreInfo record from a Rust struct.
pub async fn add_guild_raid_battle_score_info(
    pool: &SqlitePool,
    data: &GuildRaidBattleScoreInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildRaidBattleScoreInfo (
    Uid,
    DefaultScore,
    TurnBonusScore,
    GolemLevelBonusScore,
    SupportBonusScore,
    GuildTotalScore
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
    .bind(&data.default_score)
    .bind(&data.turn_bonus_score)
    .bind(&data.golem_level_bonus_score)
    .bind(&data.support_bonus_score)
    .bind(&data.guild_total_score)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_raid_battle_score_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildRaidBattleScoreInfo>> {
    sqlx::query_as::<_, GuildRaidBattleScoreInfo>(
        "SELECT * FROM GuildRaidBattleScoreInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all GuildRaidBattleScoreInfo rows for a UID.
pub async fn delete_guild_raid_battle_score_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildRaidBattleScoreInfo WHERE Uid = ?")
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
) -> sqlx::Result<GuildRaidBattleScoreInfo> {
    sqlx::query_as::<_, GuildRaidBattleScoreInfo>(
        "SELECT * FROM GuildRaidBattleScoreInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GuildRaidBattleScoreInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildRaidBattleScoreInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildRaidBattleScoreInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildRaidBattleScoreInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildRaidBattleScoreInfo (
    Uid,
    DefaultScore,
    TurnBonusScore,
    GolemLevelBonusScore,
    SupportBonusScore,
    GuildTotalScore
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
    .bind(&data.default_score)
    .bind(&data.turn_bonus_score)
    .bind(&data.golem_level_bonus_score)
    .bind(&data.support_bonus_score)
    .bind(&data.guild_total_score)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
