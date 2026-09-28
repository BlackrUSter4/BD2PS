use crate::models::game::battle::battle_char_info::BattleCharInfo;
use sqlx::SqlitePool;

/// Add a single BattleCharInfo record from a Rust struct.
pub async fn add_battle_char_info(pool: &SqlitePool, data: &BattleCharInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO BattleCharInfo (
    Uid,
    UniqueIndex,
    InvenIndex,
    Id,
    Hp,
    Level,
    CostumeInvenIndex,
    CostumeId,
    CostumeLevel,
    GridIndex,
    ReserveCostumeId,
    IsActiveSubSkillUse,
    BuffPlusStat,
    BuffMultipleStat,
    AttackDamage,
    BattlePower,
    TotalWarPlayType,
    ConnectPotentialCostume,
    Key,
    Value,
    TargetingCount,
    SupporterOwnerIndex,
    SupporterSlotIndex,
    CostumeDesignId
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
    .bind(&data.unique_index)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.hp)
    .bind(&data.level)
    .bind(&data.costume_inven_index)
    .bind(&data.costume_id)
    .bind(&data.costume_level)
    .bind(&data.grid_index)
    .bind(&data.reserve_costume_id)
    .bind(&data.is_active_sub_skill_use)
    .bind(&data.buff_plus_stat)
    .bind(&data.buff_multiple_stat)
    .bind(&data.attack_damage)
    .bind(&data.battle_power)
    .bind(&data.total_war_play_type)
    .bind(&data.connect_potential_costume)
    .bind(&data.key)
    .bind(&data.value)
    .bind(&data.targeting_count)
    .bind(&data.supporter_owner_index)
    .bind(&data.supporter_slot_index)
    .bind(&data.costume_design_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_battle_char_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<BattleCharInfo>> {
    sqlx::query_as::<_, BattleCharInfo>("SELECT * FROM BattleCharInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all BattleCharInfo rows for a UID.
pub async fn delete_battle_char_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM BattleCharInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<BattleCharInfo> {
    sqlx::query_as::<_, BattleCharInfo>("SELECT * FROM BattleCharInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<BattleCharInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM BattleCharInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, BattleCharInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &BattleCharInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO BattleCharInfo (
    Uid,
    UniqueIndex,
    InvenIndex,
    Id,
    Hp,
    Level,
    CostumeInvenIndex,
    CostumeId,
    CostumeLevel,
    GridIndex,
    ReserveCostumeId,
    IsActiveSubSkillUse,
    BuffPlusStat,
    BuffMultipleStat,
    AttackDamage,
    BattlePower,
    TotalWarPlayType,
    ConnectPotentialCostume,
    Key,
    Value,
    TargetingCount,
    SupporterOwnerIndex,
    SupporterSlotIndex,
    CostumeDesignId
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
    .bind(&data.unique_index)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.hp)
    .bind(&data.level)
    .bind(&data.costume_inven_index)
    .bind(&data.costume_id)
    .bind(&data.costume_level)
    .bind(&data.grid_index)
    .bind(&data.reserve_costume_id)
    .bind(&data.is_active_sub_skill_use)
    .bind(&data.buff_plus_stat)
    .bind(&data.buff_multiple_stat)
    .bind(&data.attack_damage)
    .bind(&data.battle_power)
    .bind(&data.total_war_play_type)
    .bind(&data.connect_potential_costume)
    .bind(&data.key)
    .bind(&data.value)
    .bind(&data.targeting_count)
    .bind(&data.supporter_owner_index)
    .bind(&data.supporter_slot_index)
    .bind(&data.costume_design_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
