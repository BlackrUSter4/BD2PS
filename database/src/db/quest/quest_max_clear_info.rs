use crate::models::game::quest::quest_max_clear_info::QuestMaxClearInfo;
use sqlx::SqlitePool;

/// Add a single QuestMaxClearInfo record from a Rust struct.
pub async fn add_quest_max_clear_info(
    pool: &SqlitePool,
    data: &QuestMaxClearInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO QuestMaxClearInfo (
    Uid,
    PackId,
    MaxClearId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.max_clear_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_quest_max_clear_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<QuestMaxClearInfo>> {
    sqlx::query_as::<_, QuestMaxClearInfo>("SELECT * FROM QuestMaxClearInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all QuestMaxClearInfo rows for a UID.
pub async fn delete_quest_max_clear_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM QuestMaxClearInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn upsert_quest_max_clear_info(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
    max_clear_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO QuestMaxClearInfo (Uid, PackId, MaxClearId)
        VALUES (?, ?, ?)
        ON CONFLICT(Uid, PackId)
        DO UPDATE SET MaxClearId = excluded.MaxClearId
        "#,
    )
    .bind(uid)
    .bind(pack_id)
    .bind(max_clear_id)
    .execute(pool)
    .await?;
    Ok(())
}
