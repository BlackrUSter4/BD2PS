use crate::models::game::overwhelm::overwhelm_monster_info::OverwhelmMonsterInfo;
use sqlx::SqlitePool;

/// Add a single OverwhelmMonsterInfo record from a Rust struct.
pub async fn add_overwhelm_monster_info(
    pool: &SqlitePool,
    data: &OverwhelmMonsterInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO OverwhelmMonsterInfo (
    Uid,
    GroupId,
    MonsterId,
    BattleDeck,
    BattleMode
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.monster_id)
    .bind(&data.battle_deck)
    .bind(&data.battle_mode)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_overwhelm_monster_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<OverwhelmMonsterInfo>> {
    sqlx::query_as::<_, OverwhelmMonsterInfo>("SELECT * FROM OverwhelmMonsterInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all OverwhelmMonsterInfo rows for a UID.
pub async fn delete_overwhelm_monster_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM OverwhelmMonsterInfo WHERE Uid = ?")
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
) -> sqlx::Result<OverwhelmMonsterInfo> {
    sqlx::query_as::<_, OverwhelmMonsterInfo>(
        "SELECT * FROM OverwhelmMonsterInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<OverwhelmMonsterInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM OverwhelmMonsterInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, OverwhelmMonsterInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &OverwhelmMonsterInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO OverwhelmMonsterInfo (
    Uid,
    GroupId,
    MonsterId,
    BattleDeck,
    BattleMode
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.monster_id)
    .bind(&data.battle_deck)
    .bind(&data.battle_mode)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
