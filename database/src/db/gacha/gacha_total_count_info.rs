use crate::models::game::gacha::gacha_total_count_info::GachaTotalCountInfo;
use sqlx::SqlitePool;

/// Add a single GachaTotalCountInfo record from a Rust struct.
pub async fn add_gacha_total_count_info(
    pool: &SqlitePool,
    data: &GachaTotalCountInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaTotalCountInfo (
    Uid,
    GachaLogType,
    Char5PickUpCostume,
    Char5Costume,
    Char4Costume,
    Char3Costume,
    Char5PickUpEquip4,
    Char5Equip4,
    Char5Equip3,
    Char4Equip4,
    Char4Equip3,
    Char4Equip2,
    Char3Equip4,
    Char3Equip3,
    Char3Equip2
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.gacha_log_type)
    .bind(&data.char5_pick_up_costume)
    .bind(&data.char5_costume)
    .bind(&data.char4_costume)
    .bind(&data.char3_costume)
    .bind(&data.char5_pick_up_equip4)
    .bind(&data.char5_equip4)
    .bind(&data.char5_equip3)
    .bind(&data.char4_equip4)
    .bind(&data.char4_equip3)
    .bind(&data.char4_equip2)
    .bind(&data.char3_equip4)
    .bind(&data.char3_equip3)
    .bind(&data.char3_equip2)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_total_count_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GachaTotalCountInfo>> {
    sqlx::query_as::<_, GachaTotalCountInfo>("SELECT * FROM GachaTotalCountInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GachaTotalCountInfo rows for a UID.
pub async fn delete_gacha_total_count_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaTotalCountInfo WHERE Uid = ?")
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
) -> sqlx::Result<GachaTotalCountInfo> {
    sqlx::query_as::<_, GachaTotalCountInfo>(
        "SELECT * FROM GachaTotalCountInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GachaTotalCountInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GachaTotalCountInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GachaTotalCountInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GachaTotalCountInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GachaTotalCountInfo (
    Uid,
    GachaLogType,
    Char5PickUpCostume,
    Char5Costume,
    Char4Costume,
    Char3Costume,
    Char5PickUpEquip4,
    Char5Equip4,
    Char5Equip3,
    Char4Equip4,
    Char4Equip3,
    Char4Equip2,
    Char3Equip4,
    Char3Equip3,
    Char3Equip2
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.gacha_log_type)
    .bind(&data.char5_pick_up_costume)
    .bind(&data.char5_costume)
    .bind(&data.char4_costume)
    .bind(&data.char3_costume)
    .bind(&data.char5_pick_up_equip4)
    .bind(&data.char5_equip4)
    .bind(&data.char5_equip3)
    .bind(&data.char4_equip4)
    .bind(&data.char4_equip3)
    .bind(&data.char4_equip2)
    .bind(&data.char3_equip4)
    .bind(&data.char3_equip3)
    .bind(&data.char3_equip2)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
