use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::pvp::pvp_battle_history_deck_info::PvpBattleHistoryDeckInfo;
/// Insert a full JSON array of PvpBattleHistoryDeckInfo records for a UID.
pub async fn insert_pvp_battle_history_deck_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("pvpBattleHistoryDeckInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_pvp_battle_history_deck_info: missing or invalid 'pvpBattleHistoryDeckInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle single nested PvpBattleUserDeckFullInfo - extract InvenIndex
        let user_deck_full_info_index = entry
            .get("userDeckFullInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());
        // Handle single nested PvpBattleUserDeckFullInfo - extract InvenIndex
        let enemy_deck_full_info_index = entry
            .get("enemyDeckFullInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO PvpBattleHistoryDeckInfo (
    Uid,
    UserDeckFullInfoIndex,
    EnemyDeckFullInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&user_deck_full_info_index)
        .bind(&enemy_deck_full_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single PvpBattleHistoryDeckInfo record from a Rust struct.
pub async fn add_pvp_battle_history_deck_info(pool: &SqlitePool, data: &PvpBattleHistoryDeckInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleHistoryDeckInfo (
    Uid,
    UserDeckFullInfoIndex,
    EnemyDeckFullInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.user_deck_full_info_index)
    .bind(&data.enemy_deck_full_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_history_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PvpBattleHistoryDeckInfo>> {
    sqlx::query_as::<_, PvpBattleHistoryDeckInfo>("SELECT * FROM PvpBattleHistoryDeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PvpBattleHistoryDeckInfo rows for a UID.
pub async fn delete_pvp_battle_history_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleHistoryDeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}