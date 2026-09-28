use crate::models::game::tactics_bingo::tactics_bingo_clear_info::TacticsBingoClearInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64, event_schedule_id: i32, group_id: i32) -> TacticsBingoClearInfo {
    sqlx::query_as::<_, TacticsBingoClearInfo>(
        "SELECT * FROM TacticsBingoClearInfo WHERE Uid = ? AND EventScheduleId = ? AND GroupId = ?",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(group_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(TacticsBingoClearInfo {
        uid,
        event_schedule_id,
        group_id,
        clear_stage: None,
        event_flag: None,
    })
}

pub async fn save(pool: &SqlitePool, info: &TacticsBingoClearInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TacticsBingoClearInfo (Uid, EventScheduleId, GroupId, ClearStage, EventFlag)
VALUES (?, ?, ?, ?, ?)
ON CONFLICT(Uid, EventScheduleId, GroupId) DO UPDATE SET
    ClearStage = excluded.ClearStage,
    EventFlag = excluded.EventFlag
"#,
    )
    .bind(info.uid)
    .bind(info.event_schedule_id)
    .bind(info.group_id)
    .bind(&info.clear_stage)
    .bind(info.event_flag)
    .execute(pool)
    .await?;
    Ok(())
}
