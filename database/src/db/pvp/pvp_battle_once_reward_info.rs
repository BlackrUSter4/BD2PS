use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::pvp::pvp_battle_once_reward_info::PvpBattleOnceRewardInfo;
/// Insert a full JSON array of PvpBattleOnceRewardInfo records for a UID.
pub async fn insert_pvp_battle_once_reward_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("pvpBattleOnceRewardInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_pvp_battle_once_reward_info: missing or invalid 'pvpBattleOnceRewardInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated primitive fields - insert one row per value
        let once_reward_info_array = entry.get("onceRewardInfo").and_then(|v| v.as_array());
        if let Some(values) = once_reward_info_array {
            for item in values {
                let once_reward_info = item.as_i64().unwrap_or_default() as i32;

                sqlx::query(
                    r#"
INSERT INTO PvpBattleOnceRewardInfo (
    Uid,
    OnceRewardInfo
) VALUES (
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&once_reward_info)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Add a single PvpBattleOnceRewardInfo record from a Rust struct.
pub async fn add_pvp_battle_once_reward_info(pool: &SqlitePool, data: &PvpBattleOnceRewardInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleOnceRewardInfo (
    Uid,
    OnceRewardInfo
) VALUES (
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.once_reward_info)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_once_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PvpBattleOnceRewardInfo>> {
    sqlx::query_as::<_, PvpBattleOnceRewardInfo>("SELECT * FROM PvpBattleOnceRewardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PvpBattleOnceRewardInfo rows for a UID.
pub async fn delete_pvp_battle_once_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleOnceRewardInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}