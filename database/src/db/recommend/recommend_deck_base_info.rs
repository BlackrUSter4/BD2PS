use crate::models::game::recommend::recommend_deck_base_info::RecommendDeckBaseInfo;
use sqlx::SqlitePool;

/// Add a single RecommendDeckBaseInfo record from a Rust struct.
pub async fn add_recommend_deck_base_info(
    pool: &SqlitePool,
    data: &RecommendDeckBaseInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO RecommendDeckBaseInfo (
    Uid,
    BattlePower,
    DeckInfoIndex,
    TotalWarDeckInfoIndex,
    CharInfoIndex,
    CostumeInfoIndex,
    EquipInfoIndex,
    AwakeInfoIndex
) VALUES (
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
    .bind(&data.battle_power)
    .bind(&data.deck_info_index)
    .bind(&data.total_war_deck_info_index)
    .bind(&data.char_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.equip_info_index)
    .bind(&data.awake_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_recommend_deck_base_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<RecommendDeckBaseInfo>> {
    sqlx::query_as::<_, RecommendDeckBaseInfo>("SELECT * FROM RecommendDeckBaseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all RecommendDeckBaseInfo rows for a UID.
pub async fn delete_recommend_deck_base_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM RecommendDeckBaseInfo WHERE Uid = ?")
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
) -> sqlx::Result<RecommendDeckBaseInfo> {
    sqlx::query_as::<_, RecommendDeckBaseInfo>(
        "SELECT * FROM RecommendDeckBaseInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<RecommendDeckBaseInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM RecommendDeckBaseInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, RecommendDeckBaseInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &RecommendDeckBaseInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO RecommendDeckBaseInfo (
    Uid,
    BattlePower,
    DeckInfoIndex,
    TotalWarDeckInfoIndex,
    CharInfoIndex,
    CostumeInfoIndex,
    EquipInfoIndex,
    AwakeInfoIndex
) VALUES (
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
    .bind(&data.battle_power)
    .bind(&data.deck_info_index)
    .bind(&data.total_war_deck_info_index)
    .bind(&data.char_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.equip_info_index)
    .bind(&data.awake_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
