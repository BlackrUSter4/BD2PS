use crate::models::game::monster::monster_hunt_user_info::MonsterHuntUserInfo;
use sqlx::SqlitePool;

/// Add a single MonsterHuntUserInfo record from a Rust struct.
pub async fn add_monster_hunt_user_info(
    pool: &SqlitePool,
    data: &MonsterHuntUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterHuntUserInfo (
    Uid,
    Season,
    MonsterHuntId,
    Level,
    StartHp,
    HighestFirstTurnDamage,
    HighestHp,
    HighestHpDate,
    CurrentLevelHighestDamage,
    DailyHighestDamage,
    SeasonReward,
    DailyRewardLevel,
    DailyRewardDate
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
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.season)
    .bind(&data.monster_hunt_id)
    .bind(&data.level)
    .bind(&data.start_hp)
    .bind(&data.highest_first_turn_damage)
    .bind(&data.highest_hp)
    .bind(&data.highest_hp_date)
    .bind(&data.current_level_highest_damage)
    .bind(&data.daily_highest_damage)
    .bind(&data.season_reward)
    .bind(&data.daily_reward_level)
    .bind(&data.daily_reward_date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_hunt_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterHuntUserInfo>> {
    sqlx::query_as::<_, MonsterHuntUserInfo>("SELECT * FROM MonsterHuntUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the account's single MonsterHuntUserInfo row, creating a fresh one if this account
/// has never fought the monster hunt boss before. `default` supplies the starting values
/// (level 1, boss id, starting hp) since a brand-new row needs real starting stats.
pub async fn get_or_create(
    pool: &SqlitePool,
    uid: i64,
    default: impl FnOnce() -> MonsterHuntUserInfo,
) -> sqlx::Result<MonsterHuntUserInfo> {
    if let Some(row) = get_monster_hunt_user_info(pool, uid).await?.into_iter().next() {
        return Ok(row);
    }
    let fresh = default();
    add_monster_hunt_user_info(pool, &fresh).await?;
    Ok(get_monster_hunt_user_info(pool, uid)
        .await?
        .into_iter()
        .next()
        .expect("row was just inserted"))
}

/// Full-row update, matched by Uid (there is exactly one MonsterHuntUserInfo row per account).
pub async fn update_monster_hunt_user_info(
    pool: &SqlitePool,
    data: &MonsterHuntUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
UPDATE MonsterHuntUserInfo SET
    Season = ?,
    MonsterHuntId = ?,
    Level = ?,
    StartHp = ?,
    HighestFirstTurnDamage = ?,
    HighestHp = ?,
    HighestHpDate = ?,
    CurrentLevelHighestDamage = ?,
    DailyHighestDamage = ?,
    SeasonReward = ?,
    DailyRewardLevel = ?,
    DailyRewardDate = ?
WHERE Uid = ?
"#,
    )
    .bind(&data.season)
    .bind(&data.monster_hunt_id)
    .bind(&data.level)
    .bind(&data.start_hp)
    .bind(&data.highest_first_turn_damage)
    .bind(&data.highest_hp)
    .bind(&data.highest_hp_date)
    .bind(&data.current_level_highest_damage)
    .bind(&data.daily_highest_damage)
    .bind(&data.season_reward)
    .bind(&data.daily_reward_level)
    .bind(&data.daily_reward_date)
    .bind(&data.uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Real cross-account ranking: every account's MonsterHuntUserInfo row, ordered by
/// progress (level first, then highest damage within that level).
pub async fn rank_all(pool: &SqlitePool, limit: i64) -> sqlx::Result<Vec<MonsterHuntUserInfo>> {
    sqlx::query_as::<_, MonsterHuntUserInfo>(
        "SELECT * FROM MonsterHuntUserInfo ORDER BY Level DESC, CurrentLevelHighestDamage DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Total number of accounts that have a MonsterHuntUserInfo row (for rank_top_percent).
pub async fn count_all(pool: &SqlitePool) -> sqlx::Result<i64> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM MonsterHuntUserInfo")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// Delete all MonsterHuntUserInfo rows for a UID.
pub async fn delete_monster_hunt_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterHuntUserInfo WHERE Uid = ?")
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
) -> sqlx::Result<MonsterHuntUserInfo> {
    sqlx::query_as::<_, MonsterHuntUserInfo>(
        "SELECT * FROM MonsterHuntUserInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MonsterHuntUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterHuntUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterHuntUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterHuntUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterHuntUserInfo (
    Uid,
    Season,
    MonsterHuntId,
    Level,
    StartHp,
    HighestFirstTurnDamage,
    HighestHp,
    HighestHpDate,
    CurrentLevelHighestDamage,
    DailyHighestDamage,
    SeasonReward,
    DailyRewardLevel,
    DailyRewardDate
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
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.season)
    .bind(&data.monster_hunt_id)
    .bind(&data.level)
    .bind(&data.start_hp)
    .bind(&data.highest_first_turn_damage)
    .bind(&data.highest_hp)
    .bind(&data.highest_hp_date)
    .bind(&data.current_level_highest_damage)
    .bind(&data.daily_highest_damage)
    .bind(&data.season_reward)
    .bind(&data.daily_reward_level)
    .bind(&data.daily_reward_date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
