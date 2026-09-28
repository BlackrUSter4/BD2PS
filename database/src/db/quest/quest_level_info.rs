use crate::models::game::quest::quest_level_info::QuestLevelInfo;
use sqlx::SqlitePool;

/// Add a single QuestLevelInfo record from a Rust struct.
pub async fn add_quest_level_info(pool: &SqlitePool, data: &QuestLevelInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO QuestLevelInfo (
    Uid,
    PackId,
    QuestLevel,
    ClearQuest,
    QuestOpt,
    IsLevelComplete
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
    .bind(&data.pack_id)
    .bind(&data.quest_level)
    .bind(&data.clear_quest)
    .bind(&data.quest_opt)
    .bind(&data.is_level_complete)
    .execute(pool)
    .await?;

    Ok(())
}

/// Upsert (insert or update) a QuestLevelInfo record for a given UID and PackId.
pub async fn upsert_quest_level_info(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
    clear_quest: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO QuestLevelInfo (
    Uid,
    PackId,
    QuestLevel,
    ClearQuest,
    QuestOpt,
    IsLevelComplete
)
VALUES (?, ?, 0, ?, 0, 0)
ON CONFLICT(Uid, PackId)
DO UPDATE SET
    ClearQuest = excluded.ClearQuest
"#,
    )
    .bind(uid)
    .bind(pack_id)
    .bind(clear_quest)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_quest_level_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<QuestLevelInfo>> {
    sqlx::query_as::<_, QuestLevelInfo>("SELECT * FROM QuestLevelInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all QuestLevelInfo rows for a UID.
pub async fn delete_quest_level_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM QuestLevelInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
