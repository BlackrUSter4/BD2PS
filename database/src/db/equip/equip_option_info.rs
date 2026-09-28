use crate::models::game::equip::equip_option_info::EquipOptionInfo;
use sqlx::SqlitePool;

/// Add a single EquipOptionInfo record from a Rust struct.
pub async fn add_equip_option_info(pool: &SqlitePool, data: &EquipOptionInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EquipOptionInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_equip_option_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EquipOptionInfo>> {
    sqlx::query_as::<_, EquipOptionInfo>("SELECT * FROM EquipOptionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EquipOptionInfo rows for a UID.
pub async fn delete_equip_option_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EquipOptionInfo WHERE Uid = ?")
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
) -> sqlx::Result<EquipOptionInfo> {
    sqlx::query_as::<_, EquipOptionInfo>(
        "SELECT * FROM EquipOptionInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EquipOptionInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EquipOptionInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EquipOptionInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EquipOptionInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EquipOptionInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
