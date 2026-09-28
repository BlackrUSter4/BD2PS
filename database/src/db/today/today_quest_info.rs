use crate::models::game::today::today_quest_info::TodayQuestInfo;
use sqlx::SqlitePool;

/// Add a single TodayQuestInfo record from a Rust struct.
pub async fn add_today_quest_info(pool: &SqlitePool, data: &TodayQuestInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TodayQuestInfo (
    Uid,
    QuestInfoIndex,
    ClearQuestIds,
    TodayEndTime,
    TodayQuestId
) VALUES (?, ?, ?, ?, ?)
"#,
    )
    .bind(&data.uid)
    .bind(&data.quest_info_index)
    .bind(&data.clear_quest_ids)
    .bind(&data.today_end_time)
    .bind(&data.today_quest_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_today_quest_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<TodayQuestInfo>> {
    sqlx::query_as::<_, TodayQuestInfo>(
        r#"
        SELECT
            "Index",
            "Uid",
            "QuestInfoIndex",
            "ClearQuestIds",
            "TodayEndTime",
            "TodayQuestId"
        FROM TodayQuestInfo
        WHERE Uid = ?
        ORDER BY "Index" ASC
        "#,
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all TodayQuestInfo rows for a UID.
pub async fn delete_today_quest_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TodayQuestInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
