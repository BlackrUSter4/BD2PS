use crate::models::game::friendship::friendship_info::FriendshipInfo;
use sqlx::SqlitePool;

/// No `FriendshipLevelTable`/exp-curve data was captured for this feature — level-up uses a
/// documented placeholder linear curve (100 exp * current level per level-up), capped at
/// `friendshipMaxLevel3` (20) from `FriendshipDefaultTable`, the highest of the three tier caps
/// that table defines (per-costume tier assignment isn't captured either, so the safest single
/// cap is used for everyone).
pub const MAX_LEVEL: i32 = 20;

pub async fn get_or_default(pool: &SqlitePool, uid: i64, costume_id: i32) -> FriendshipInfo {
    sqlx::query_as::<_, FriendshipInfo>("SELECT * FROM FriendshipInfo WHERE Uid = ? AND CostumeId = ?")
        .bind(uid)
        .bind(costume_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(FriendshipInfo::new_default(uid, costume_id))
}

pub async fn get_all(pool: &SqlitePool, uid: i64) -> Vec<FriendshipInfo> {
    sqlx::query_as::<_, FriendshipInfo>("SELECT * FROM FriendshipInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}

/// Adds `exp_gain` and applies level-ups (see `MAX_LEVEL` doc comment), then persists.
/// Returns the updated row.
pub async fn add_exp(pool: &SqlitePool, uid: i64, costume_id: i32, exp_gain: i32, touch_counseling_date: bool) -> FriendshipInfo {
    let mut info = get_or_default(pool, uid, costume_id).await;
    info.exp += exp_gain;
    while info.level < MAX_LEVEL {
        let required = info.level * 100;
        if info.exp < required {
            break;
        }
        info.exp -= required;
        info.level += 1;
    }
    if touch_counseling_date {
        info.last_counseling_date = Some(chrono::Utc::now().timestamp_millis());
    }
    save(pool, &info).await;
    info
}

async fn save(pool: &SqlitePool, info: &FriendshipInfo) {
    let _ = sqlx::query(
        r#"
INSERT INTO FriendshipInfo (Uid, CostumeId, Level, Exp, LastCounselingDate)
VALUES (?, ?, ?, ?, ?)
ON CONFLICT(Uid, CostumeId) DO UPDATE SET
    Level = excluded.Level,
    Exp = excluded.Exp,
    LastCounselingDate = excluded.LastCounselingDate
"#,
    )
    .bind(info.uid)
    .bind(info.costume_id)
    .bind(info.level)
    .bind(info.exp)
    .bind(info.last_counseling_date)
    .execute(pool)
    .await;
}
