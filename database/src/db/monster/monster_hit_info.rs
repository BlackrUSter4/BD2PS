use crate::models::game::monster::monster_hit_info::MonsterHitInfo;
use sqlx::SqlitePool;

/// Add a single MonsterHitInfo record from a Rust struct.
pub async fn add_monster_hit_info(pool: &SqlitePool, data: &MonsterHitInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterHitInfo (
    Uid,
    RageValue,
    GroggyValue,
    TargetPartsId,
    TargetAttackType,
    Damage,
    AttackOwnerIndex,
    ContactPoint,
    DisplayAttackCount,
    IsCritical,
    IsWeak
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
    .bind(&data.rage_value)
    .bind(&data.groggy_value)
    .bind(&data.target_parts_id)
    .bind(&data.target_attack_type)
    .bind(&data.damage)
    .bind(&data.attack_owner_index)
    .bind(&data.contact_point_index)
    .bind(&data.display_attack_count)
    .bind(&data.is_critical)
    .bind(&data.is_weak)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_hit_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterHitInfo>> {
    sqlx::query_as::<_, MonsterHitInfo>("SELECT * FROM MonsterHitInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MonsterHitInfo rows for a UID.
pub async fn delete_monster_hit_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterHitInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MonsterHitInfo> {
    sqlx::query_as::<_, MonsterHitInfo>("SELECT * FROM MonsterHitInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MonsterHitInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterHitInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterHitInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterHitInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterHitInfo (
    Uid,
    RageValue,
    GroggyValue,
    TargetPartsId,
    TargetAttackType,
    Damage,
    AttackOwnerIndex,
    ContactPoint,
    DisplayAttackCount,
    IsCritical,
    IsWeak
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
    .bind(&data.rage_value)
    .bind(&data.groggy_value)
    .bind(&data.target_parts_id)
    .bind(&data.target_attack_type)
    .bind(&data.damage)
    .bind(&data.attack_owner_index)
    .bind(&data.contact_point_index)
    .bind(&data.display_attack_count)
    .bind(&data.is_critical)
    .bind(&data.is_weak)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
