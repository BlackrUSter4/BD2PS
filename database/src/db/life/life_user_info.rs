use crate::models::game::life::life_user_info::LifeUserInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<LifeUserInfo>> {
    sqlx::query_as::<_, LifeUserInfo>("SELECT * FROM LifeUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// Fetch the row, creating a default one first if it doesn't exist yet.
pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<LifeUserInfo> {
    if let Some(existing) = get(pool, uid).await? {
        return Ok(existing);
    }
    let data = LifeUserInfo {
        uid,
        ..Default::default()
    };
    insert(pool, &data).await?;
    Ok(data)
}

pub async fn insert(pool: &SqlitePool, data: &LifeUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO LifeUserInfo (
    Uid, LifeCoin, LifeWorldId, LoggingLevel, LoggingExp, MiningLevel, MiningExp, FarmingLevel, FarmingExp
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.life_coin)
    .bind(data.life_world_id)
    .bind(data.logging_level)
    .bind(data.logging_exp)
    .bind(data.mining_level)
    .bind(data.mining_exp)
    .bind(data.farming_level)
    .bind(data.farming_exp)
    .execute(pool)
    .await?;
    Ok(())
}

/// Add (or subtract, if negative) life coin. Returns the new balance.
pub async fn add_coin(pool: &SqlitePool, uid: i64, delta: i32) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE LifeUserInfo SET LifeCoin = LifeCoin + ? WHERE Uid = ?")
        .bind(delta)
        .bind(uid)
        .execute(pool)
        .await?;
    let row: (i32,) = sqlx::query_as("SELECT LifeCoin FROM LifeUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn set_world_id(pool: &SqlitePool, uid: i64, world_id: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE LifeUserInfo SET LifeWorldId = ? WHERE Uid = ?")
        .bind(world_id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn save_char_level(
    pool: &SqlitePool,
    uid: i64,
    logging_level: i32,
    logging_exp: i32,
    mining_level: i32,
    mining_exp: i32,
    farming_level: i32,
    farming_exp: i32,
) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query(
        r#"
UPDATE LifeUserInfo SET
    LoggingLevel = ?, LoggingExp = ?, MiningLevel = ?, MiningExp = ?, FarmingLevel = ?, FarmingExp = ?
WHERE Uid = ?
"#,
    )
    .bind(logging_level)
    .bind(logging_exp)
    .bind(mining_level)
    .bind(mining_exp)
    .bind(farming_level)
    .bind(farming_exp)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}
