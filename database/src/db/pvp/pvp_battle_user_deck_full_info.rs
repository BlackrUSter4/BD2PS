use crate::models::game::pvp::pvp_battle_user_deck_full_info::PvpBattleUserDeckFullInfo;
use sqlx::SqlitePool;

/// Add a single PvpBattleUserDeckFullInfo record from a Rust struct.
pub async fn add_pvp_battle_user_deck_full_info(
    pool: &SqlitePool,
    data: &PvpBattleUserDeckFullInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleUserDeckFullInfo (
    Uid,
    CharInfoIndex,
    CostumeInfoIndex,
    EquipInfoIndex,
    BuffStatInfoIndex,
    AwakeInfoIndex
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
    .bind(&data.char_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.equip_info_index)
    .bind(&data.buff_stat_info_index)
    .bind(&data.awake_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_user_deck_full_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PvpBattleUserDeckFullInfo>> {
    sqlx::query_as::<_, PvpBattleUserDeckFullInfo>(
        "SELECT * FROM PvpBattleUserDeckFullInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all PvpBattleUserDeckFullInfo rows for a UID.
pub async fn delete_pvp_battle_user_deck_full_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleUserDeckFullInfo WHERE Uid = ?")
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
) -> sqlx::Result<PvpBattleUserDeckFullInfo> {
    sqlx::query_as::<_, PvpBattleUserDeckFullInfo>(
        "SELECT * FROM PvpBattleUserDeckFullInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PvpBattleUserDeckFullInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PvpBattleUserDeckFullInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PvpBattleUserDeckFullInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PvpBattleUserDeckFullInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PvpBattleUserDeckFullInfo (
    Uid,
    CharInfoIndex,
    CostumeInfoIndex,
    EquipInfoIndex,
    BuffStatInfoIndex,
    AwakeInfoIndex
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
    .bind(&data.char_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.equip_info_index)
    .bind(&data.buff_stat_info_index)
    .bind(&data.awake_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
