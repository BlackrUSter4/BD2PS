use crate::models::game::monster::monster_hunt_deck_info::MonsterHuntDeckInfo;
use sqlx::SqlitePool;

/// Add a single MonsterHuntDeckInfo record from a Rust struct.
pub async fn add_monster_hunt_deck_info(
    pool: &SqlitePool,
    data: &MonsterHuntDeckInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterHuntDeckInfo (
    Uid,
    Team,
    DeckInfoIndex,
    BattlePower
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.team)
    .bind(&data.deck_info_index)
    .bind(&data.battle_power)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_hunt_deck_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterHuntDeckInfo>> {
    sqlx::query_as::<_, MonsterHuntDeckInfo>("SELECT * FROM MonsterHuntDeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the row for a specific team, if it's been saved.
pub async fn get_by_uid_and_team(
    pool: &SqlitePool,
    uid: i64,
    team: i32,
) -> sqlx::Result<Option<MonsterHuntDeckInfo>> {
    sqlx::query_as::<_, MonsterHuntDeckInfo>(
        "SELECT * FROM MonsterHuntDeckInfo WHERE Uid = ? AND Team = ?",
    )
    .bind(uid)
    .bind(team)
    .fetch_optional(pool)
    .await
}

/// Insert or overwrite the deck for a given team.
pub async fn upsert(
    pool: &SqlitePool,
    uid: i64,
    team: i32,
    deck_info_index: Option<&str>,
    battle_power: Option<i32>,
) -> sqlx::Result<()> {
    if get_by_uid_and_team(pool, uid, team).await?.is_some() {
        sqlx::query(
            "UPDATE MonsterHuntDeckInfo SET DeckInfoIndex = ?, BattlePower = ? WHERE Uid = ? AND Team = ?",
        )
        .bind(deck_info_index)
        .bind(battle_power)
        .bind(uid)
        .bind(team)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO MonsterHuntDeckInfo (Uid, Team, DeckInfoIndex, BattlePower) VALUES (?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(team)
        .bind(deck_info_index)
        .bind(battle_power)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Delete all MonsterHuntDeckInfo rows for a UID.
pub async fn delete_monster_hunt_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterHuntDeckInfo WHERE Uid = ?")
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
) -> sqlx::Result<MonsterHuntDeckInfo> {
    sqlx::query_as::<_, MonsterHuntDeckInfo>(
        "SELECT * FROM MonsterHuntDeckInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MonsterHuntDeckInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterHuntDeckInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterHuntDeckInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterHuntDeckInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterHuntDeckInfo (
    Uid,
    Team,
    DeckInfoIndex,
    BattlePower
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.team)
    .bind(&data.deck_info_index)
    .bind(&data.battle_power)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
