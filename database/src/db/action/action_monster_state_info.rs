use crate::models::game::action::action_monster_state_info::ActionMonsterStateInfo;
use sqlx::SqlitePool;

/// Add a single ActionMonsterStateInfo record from a Rust struct.
pub async fn add_action_monster_state_info(
    pool: &SqlitePool,
    data: &ActionMonsterStateInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ActionMonsterStateInfo (
    Uid,
    Type,
    Seq,
    MonsterId,
    MonsterIndex,
    MonsterPosition,
    MonsterVector,
    Speed,
    DeltaTime,
    SendTime,
    Health,
    RageValue,
    GroggyValue,
    State,
    PatternInfo,
    HitInfo,
    AttackSkillId
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
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.seq)
    .bind(&data.monster_id)
    .bind(&data.monster_index)
    .bind(&data.monster_position_index)
    .bind(&data.monster_vector_index)
    .bind(&data.speed)
    .bind(&data.delta_time)
    .bind(&data.send_time)
    .bind(&data.health)
    .bind(&data.rage_value)
    .bind(&data.groggy_value)
    .bind(&data.state)
    .bind(&data.pattern_info_index)
    .bind(&data.hit_info_index)
    .bind(&data.attack_skill_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_action_monster_state_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ActionMonsterStateInfo>> {
    sqlx::query_as::<_, ActionMonsterStateInfo>(
        "SELECT * FROM ActionMonsterStateInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all ActionMonsterStateInfo rows for a UID.
pub async fn delete_action_monster_state_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ActionMonsterStateInfo WHERE Uid = ?")
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
) -> sqlx::Result<ActionMonsterStateInfo> {
    sqlx::query_as::<_, ActionMonsterStateInfo>(
        "SELECT * FROM ActionMonsterStateInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<ActionMonsterStateInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ActionMonsterStateInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ActionMonsterStateInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ActionMonsterStateInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ActionMonsterStateInfo (
    Uid,
    Type,
    Seq,
    MonsterId,
    MonsterIndex,
    MonsterPosition,
    MonsterVector,
    Speed,
    DeltaTime,
    SendTime,
    Health,
    RageValue,
    GroggyValue,
    State,
    PatternInfo,
    HitInfo,
    AttackSkillId
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
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.seq)
    .bind(&data.monster_id)
    .bind(&data.monster_index)
    .bind(&data.monster_position_index)
    .bind(&data.monster_vector_index)
    .bind(&data.speed)
    .bind(&data.delta_time)
    .bind(&data.send_time)
    .bind(&data.health)
    .bind(&data.rage_value)
    .bind(&data.groggy_value)
    .bind(&data.state)
    .bind(&data.pattern_info_index)
    .bind(&data.hit_info_index)
    .bind(&data.attack_skill_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
