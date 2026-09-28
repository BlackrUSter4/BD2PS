use crate::models::game::hunting::hunting_ground_info::HuntingGroundInfo;
use sqlx::SqlitePool;

/// Add a single HuntingGroundInfo record from a Rust struct.
pub async fn add_hunting_ground_info(
    pool: &SqlitePool,
    data: &HuntingGroundInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO HuntingGroundInfo (
    Uid,
    IsAuto,
    CurrentId,
    HighestId,
    PackId
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
    .bind(&data.is_auto)
    .bind(&data.current_id)
    .bind(&data.highest_id)
    .bind(&data.pack_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_hunting_ground_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<HuntingGroundInfo>> {
    sqlx::query_as::<_, HuntingGroundInfo>("SELECT * FROM HuntingGroundInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all HuntingGroundInfo rows for a UID.
pub async fn delete_hunting_ground_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM HuntingGroundInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert(pool: &SqlitePool, data: &HuntingGroundInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO HuntingGroundInfo (Uid, IsAuto, CurrentId, HighestId, PackId) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(data.uid)
    .bind(data.is_auto)
    .bind(data.current_id)
    .bind(data.highest_id)
    .bind(data.pack_id)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn get_by_uid_and_pack(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
) -> sqlx::Result<Option<HuntingGroundInfo>> {
    sqlx::query_as::<_, HuntingGroundInfo>("SELECT * FROM HuntingGroundInfo WHERE Uid = ? AND PackId = ?")
        .bind(uid)
        .bind(pack_id)
        .fetch_optional(pool)
        .await
}

pub async fn update_entry(
    pool: &SqlitePool,
    index: i64,
    is_auto: Option<bool>,
    current_id: i32,
    highest_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE HuntingGroundInfo SET IsAuto = ?, CurrentId = ?, HighestId = ? WHERE \"Index\" = ?",
    )
    .bind(is_auto)
    .bind(current_id)
    .bind(highest_id)
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}
