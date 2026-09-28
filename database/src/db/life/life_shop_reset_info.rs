use crate::models::game::life::life_shop_reset_info::LifeShopResetInfo;
use sqlx::SqlitePool;

const DAY_MS: i64 = 24 * 60 * 60 * 1000;
const WEEK_MS: i64 = 7 * DAY_MS;
const MONTH_MS: i64 = 30 * DAY_MS;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<LifeShopResetInfo>> {
    sqlx::query_as::<_, LifeShopResetInfo>("SELECT * FROM LifeShopResetInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

async fn upsert(pool: &SqlitePool, data: &LifeShopResetInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO LifeShopResetInfo (Uid, DailyResetTime, WeeklyResetTime, MonthlyResetTime)
VALUES (?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET
    DailyResetTime = excluded.DailyResetTime,
    WeeklyResetTime = excluded.WeeklyResetTime,
    MonthlyResetTime = excluded.MonthlyResetTime
"#,
    )
    .bind(data.uid)
    .bind(data.daily_reset_time)
    .bind(data.weekly_reset_time)
    .bind(data.monthly_reset_time)
    .execute(pool)
    .await?;
    Ok(())
}

/// Ensure reset timestamps exist and are in the future relative to `now`, resetting
/// LifeShopBuyInfo's buy counts (via the caller) whenever a period has rolled over.
/// Returns true if any period rolled over (caller should reset buy counts).
///
/// NOTE: this uses a rolling window (next reset = now + period) rather than aligning to a
/// fixed server-day/week/month boundary — simpler to reason about for a private server and
/// avoids needing a calendar/timezone dependency, but real live-service resets are usually
/// calendar-aligned. Flagged as a deliberate simplification.
pub async fn check_and_advance(pool: &SqlitePool, uid: i64, now: i64) -> sqlx::Result<bool> {
    let existing = get(pool, uid).await?;
    let rolled_over;

    let (daily, weekly, monthly) = match existing {
        Some(row) => {
            let daily_expired = row.daily_reset_time.map(|t| now >= t).unwrap_or(true);
            let weekly_expired = row.weekly_reset_time.map(|t| now >= t).unwrap_or(true);
            let monthly_expired = row.monthly_reset_time.map(|t| now >= t).unwrap_or(true);
            rolled_over = daily_expired || weekly_expired || monthly_expired;

            (
                if daily_expired { now + DAY_MS } else { row.daily_reset_time.unwrap() },
                if weekly_expired { now + WEEK_MS } else { row.weekly_reset_time.unwrap() },
                if monthly_expired { now + MONTH_MS } else { row.monthly_reset_time.unwrap() },
            )
        }
        None => {
            rolled_over = false; // first-time init, nothing to reset
            (now + DAY_MS, now + WEEK_MS, now + MONTH_MS)
        }
    };

    upsert(
        pool,
        &LifeShopResetInfo {
            uid,
            daily_reset_time: Some(daily),
            weekly_reset_time: Some(weekly),
            monthly_reset_time: Some(monthly),
        },
    )
    .await?;

    Ok(rolled_over)
}
