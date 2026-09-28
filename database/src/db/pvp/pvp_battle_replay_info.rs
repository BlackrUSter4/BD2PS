use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::pvp::pvp_battle_replay_info::PvpBattleReplayInfo;
/// Insert a full JSON array of PvpBattleReplayInfo records for a UID.
pub async fn insert_pvp_battle_replay_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("pvpBattleReplayInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_pvp_battle_replay_info: missing or invalid 'pvpBattleReplayInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated primitive fields - insert one row per value
        let battle_random_seed_array = entry.get("battleRandomSeed").and_then(|v| v.as_array());
        if let Some(values) = battle_random_seed_array {
            for item in values {
                let blue_deck_full_info_index = entry
                    .get("blueDeckFullInfoIndex")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                let red_deck_full_info_index = entry
                    .get("redDeckFullInfoIndex")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default();
                let battle_random_seed = item.as_i64().unwrap_or_default() as i32;

                sqlx::query(
                    r#"
INSERT INTO PvpBattleReplayInfo (
    Uid,
    BlueDeckFullInfoIndex,
    RedDeckFullInfoIndex,
    BattleRandomSeed
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&blue_deck_full_info_index)
                .bind(&red_deck_full_info_index)
                .bind(&battle_random_seed)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Add a single PvpBattleReplayInfo record from a Rust struct.
pub async fn add_pvp_battle_replay_info(pool: &SqlitePool, data: &PvpBattleReplayInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleReplayInfo (
    Uid,
    BlueDeckFullInfoIndex,
    RedDeckFullInfoIndex,
    BattleRandomSeed
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.blue_deck_full_info_index)
    .bind(&data.red_deck_full_info_index)
    .bind(&data.battle_random_seed)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_replay_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PvpBattleReplayInfo>> {
    sqlx::query_as::<_, PvpBattleReplayInfo>("SELECT * FROM PvpBattleReplayInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PvpBattleReplayInfo rows for a UID.
pub async fn delete_pvp_battle_replay_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleReplayInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}