use crate::models::game::npc::npc_quiz_clear_info::NpcQuizClearInfo;
use sqlx::SqlitePool;

pub async fn get_for_event(pool: &SqlitePool, uid: i64, event_uid: i32) -> Vec<NpcQuizClearInfo> {
    sqlx::query_as::<_, NpcQuizClearInfo>("SELECT * FROM NpcQuizClearInfo WHERE Uid = ? AND EventUid = ?")
        .bind(uid)
        .bind(event_uid)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}

/// Returns true if this call newly cleared it (false if already cleared).
pub async fn try_clear(pool: &SqlitePool, uid: i64, event_uid: i32, group_id: i32, id: i32) -> bool {
    let already = sqlx::query(
        "SELECT 1 FROM NpcQuizClearInfo WHERE Uid = ? AND EventUid = ? AND GroupId = ? AND Id = ?",
    )
    .bind(uid)
    .bind(event_uid)
    .bind(group_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .is_some();
    if already {
        return false;
    }
    sqlx::query("INSERT OR IGNORE INTO NpcQuizClearInfo (Uid, EventUid, GroupId, Id) VALUES (?, ?, ?, ?)")
        .bind(uid)
        .bind(event_uid)
        .bind(group_id)
        .bind(id)
        .execute(pool)
        .await
        .is_ok()
}
