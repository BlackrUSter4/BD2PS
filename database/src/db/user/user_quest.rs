use crate::models::game::user::user_quest::UserQuest;
use sqlx::SqlitePool;

pub async fn get_user_quest(
    pool: &SqlitePool,
    uid: i64,
    quest_id: i32,
) -> sqlx::Result<Option<UserQuest>> {
    sqlx::query_as::<_, UserQuest>("SELECT * FROM UserQuest WHERE Uid = ? AND QuestId = ? LIMIT 1")
        .bind(uid)
        .bind(quest_id)
        .fetch_optional(pool)
        .await
}

pub async fn add_user_quest(pool: &SqlitePool, data: &UserQuest) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO UserQuest (Uid, QuestId, PackId, Status, Progress, RewardClaimed, LastUpdate)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(data.uid)
    .bind(data.quest_id)
    .bind(data.pack_id)
    .bind(data.status)
    .bind(data.progress)
    .bind(data.reward_claimed)
    .bind(
        data.last_update
            .unwrap_or(chrono::Utc::now().timestamp_millis()),
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<UserQuest>> {
    sqlx::query_as::<_, UserQuest>("SELECT * FROM UserQuest WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_uid_and_pack(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
) -> sqlx::Result<Vec<UserQuest>> {
    sqlx::query_as::<_, UserQuest>("SELECT * FROM UserQuest WHERE Uid = ? AND PackId = ?")
        .bind(uid)
        .bind(pack_id)
        .fetch_all(pool)
        .await
}

pub async fn delete_user_quest(pool: &SqlitePool, uid: i64, quest_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM UserQuest WHERE Uid = ? AND QuestId = ?")
        .bind(uid)
        .bind(quest_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_user_quest(pool: &SqlitePool, data: &UserQuest) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE UserQuest
        SET Status = ?, Progress = ?, RewardClaimed = ?, LastUpdate = ?
        WHERE Uid = ? AND QuestId = ?
        "#,
    )
    .bind(data.status)
    .bind(data.progress)
    .bind(data.reward_claimed)
    .bind(chrono::Utc::now().timestamp_millis())
    .bind(data.uid)
    .bind(data.quest_id)
    .execute(pool)
    .await?;
    Ok(())
}
