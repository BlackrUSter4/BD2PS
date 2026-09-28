use sqlx::{FromRow, SqlitePool};

/// Rolling report-count window: how many times an account must be reported before a penalty
/// fires, and how long the counting window / the resulting penalty lasts. No table anywhere
/// defines these — documented placeholders.
pub const REPORT_THRESHOLD: i32 = 5;
pub const REPORT_WINDOW_MS: i64 = 24 * 60 * 60 * 1000;
pub const PENALTY_DURATION_MS: i64 = 60 * 60 * 1000;

#[derive(Debug, Default, FromRow)]
pub struct RoomChatReportUser {
    pub uid: i64,
    pub report_count: i32,
    pub report_count_reset_time: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct RoomChatPenalty {
    pub start_time: i64,
    pub end_time: i64,
    pub report_id: Option<i32>,
}

pub async fn log_report(
    pool: &SqlitePool,
    reporter_uid: i64,
    target_owner_index: i64,
    report_id: Option<i32>,
    text: Option<&str>,
    reason: Option<&str>,
    chat_time: Option<i64>,
    now: i64,
) {
    let _ = sqlx::query(
        "INSERT INTO RoomChatReportLog (ReporterUid, TargetOwnerIndex, ReportId, Text, Reason, ChatTime, CreatedAt) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(reporter_uid)
    .bind(target_owner_index)
    .bind(report_id)
    .bind(text)
    .bind(reason)
    .bind(chat_time)
    .bind(now)
    .execute(pool)
    .await;
}

pub async fn get_report_user(pool: &SqlitePool, uid: i64) -> RoomChatReportUser {
    sqlx::query_as::<_, RoomChatReportUser>(
        "SELECT Uid as uid, ReportCount as report_count, ReportCountResetTime as report_count_reset_time \
         FROM RoomChatReportUser WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(RoomChatReportUser { uid, report_count: 0, report_count_reset_time: None })
}

/// Increments the target's accumulated report count (resetting the rolling window if it has
/// elapsed), and returns the count AFTER this report so the caller can decide whether a penalty
/// threshold was crossed.
pub async fn record_against_target(pool: &SqlitePool, target_uid: i64, now: i64) -> i32 {
    let current = get_report_user(pool, target_uid).await;

    let (new_count, reset_time) = match current.report_count_reset_time {
        Some(reset_time) if reset_time > now => (current.report_count + 1, reset_time),
        _ => (1, now + REPORT_WINDOW_MS),
    };

    let _ = sqlx::query(
        "INSERT INTO RoomChatReportUser (Uid, ReportCount, ReportCountResetTime) VALUES (?, ?, ?) \
         ON CONFLICT(Uid) DO UPDATE SET ReportCount = excluded.ReportCount, ReportCountResetTime = excluded.ReportCountResetTime",
    )
    .bind(target_uid)
    .bind(new_count)
    .bind(reset_time)
    .execute(pool)
    .await;

    new_count
}

pub async fn reset_report_count(pool: &SqlitePool, uid: i64) {
    let _ = sqlx::query("UPDATE RoomChatReportUser SET ReportCount = 0 WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await;
}

pub async fn add_penalty(pool: &SqlitePool, uid: i64, start_time: i64, end_time: i64, report_id: Option<i32>) {
    let _ = sqlx::query("INSERT INTO RoomChatPenalty (Uid, StartTime, EndTime, ReportId) VALUES (?, ?, ?, ?)")
        .bind(uid)
        .bind(start_time)
        .bind(end_time)
        .bind(report_id)
        .execute(pool)
        .await;
}

/// The account's currently-active penalty (EndTime in the future), if any.
pub async fn get_active_penalty(pool: &SqlitePool, uid: i64, now: i64) -> Option<RoomChatPenalty> {
    sqlx::query_as::<_, RoomChatPenalty>(
        "SELECT StartTime as start_time, EndTime as end_time, ReportId as report_id \
         FROM RoomChatPenalty WHERE Uid = ? AND EndTime > ? ORDER BY EndTime DESC LIMIT 1",
    )
    .bind(uid)
    .bind(now)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}
