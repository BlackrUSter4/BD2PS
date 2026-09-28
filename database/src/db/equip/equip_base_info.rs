use crate::models::game::equip::equip_base_info::EquipBaseInfo;
use sqlx::SqlitePool;

/// Insert and return the rowid (Index).
pub async fn insert(pool: &SqlitePool, data: &EquipBaseInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EquipBaseInfo (
    Uid,
    Id,
    Level,
    MainOptionIndex,
    SubOptionIndex,
    PrivateOptionIndex,
    Rank,
    PrevMainOptionIndex,
    PrevSubOptionIndex
) VALUES (
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
    .bind(&data.id)
    .bind(&data.level)
    .bind(&data.main_option_index)
    .bind(&data.sub_option_index)
    .bind(&data.private_option_index)
    .bind(&data.rank)
    .bind(&data.prev_main_option_index)
    .bind(&data.prev_sub_option_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<Option<EquipBaseInfo>> {
    sqlx::query_as::<_, EquipBaseInfo>("SELECT * FROM EquipBaseInfo WHERE Uid = ? AND \"Index\" = ?")
        .bind(uid)
        .bind(index)
        .fetch_optional(pool)
        .await
}

pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<EquipBaseInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EquipBaseInfo WHERE Uid = ? AND \"Index\" IN ({})",
        placeholders
    );
    let mut query = sqlx::query_as::<_, EquipBaseInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }
    query.fetch_all(pool).await
}

pub async fn set_level(pool: &SqlitePool, uid: i64, index: i64, level: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE EquipBaseInfo SET Level = ? WHERE Uid = ? AND \"Index\" = ?")
        .bind(level)
        .bind(uid)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_rank(pool: &SqlitePool, uid: i64, index: i64, rank: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE EquipBaseInfo SET Rank = ? WHERE Uid = ? AND \"Index\" = ?")
        .bind(rank)
        .bind(uid)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_options(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
    main_option_index: Option<&str>,
    sub_option_index: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE EquipBaseInfo SET MainOptionIndex = ?, SubOptionIndex = ? WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(main_option_index)
    .bind(sub_option_index)
    .bind(uid)
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}

/// Overwrites the live options and stashes what they were before, for a possible later revert.
pub async fn reroll_options(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
    prev_main: Option<&str>,
    prev_sub: Option<&str>,
    new_main: Option<&str>,
    new_sub: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE EquipBaseInfo SET MainOptionIndex = ?, SubOptionIndex = ?, PrevMainOptionIndex = ?, PrevSubOptionIndex = ? WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(new_main)
    .bind(new_sub)
    .bind(prev_main)
    .bind(prev_sub)
    .bind(uid)
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}

/// EquipOptionReRollConfirm(is_confirm=false): restore the stashed pre-reroll options.
pub async fn revert_reroll(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE EquipBaseInfo SET MainOptionIndex = PrevMainOptionIndex, SubOptionIndex = PrevSubOptionIndex, PrevMainOptionIndex = NULL, PrevSubOptionIndex = NULL WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(uid)
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}

/// EquipOptionReRollConfirm(is_confirm=true): keep the reroll, just clear the stash.
pub async fn clear_reroll_stash(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE EquipBaseInfo SET PrevMainOptionIndex = NULL, PrevSubOptionIndex = NULL WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(uid)
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EquipBaseInfo WHERE Uid = ? AND \"Index\" = ?")
        .bind(uid)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}
