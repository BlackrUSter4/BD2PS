use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::pvp::pvp_battle_deck_info::PvpBattleDeckInfo;
/// Insert PvpBattleDeckInfo data containing multiple sub-arrays for a UID.
pub async fn insert_pvp_battle_deck_info(_pool: &SqlitePool, data: &Value, _uid: i64) -> sqlx::Result<()> {
    // --- PvpBattleUserDeckInfo ---
    if let Some(arr) = data.get("attackDeckInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for PvpBattleUserDeckInfo and insert
            // This needs to be customized based on the actual fields in PvpBattleUserDeckInfo
        }
    }
    // --- ContentsCharItemInfo ---
    if let Some(arr) = data.get("attackDeckItemInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for ContentsCharItemInfo and insert
            // This needs to be customized based on the actual fields in ContentsCharItemInfo
        }
    }
    // --- PvpBattleUserDeckInfo ---
    if let Some(arr) = data.get("defenseDeckInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for PvpBattleUserDeckInfo and insert
            // This needs to be customized based on the actual fields in PvpBattleUserDeckInfo
        }
    }
    // --- ContentsCharItemInfo ---
    if let Some(arr) = data.get("defenseDeckItemInfoIndex").and_then(|v| v.as_array()) {
        for _entry in arr {
            // TODO: Extract fields for ContentsCharItemInfo and insert
            // This needs to be customized based on the actual fields in ContentsCharItemInfo
        }
    }
    Ok(())
}

/// Add a single PvpBattleDeckInfo record from a Rust struct.
pub async fn add_pvp_battle_deck_info(pool: &SqlitePool, data: &PvpBattleDeckInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleDeckInfo (
    Uid,
    AttackDeckInfoIndex,
    AttackDeckItemInfoIndex,
    DefenseDeckInfoIndex,
    DefenseDeckItemInfoIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.attack_deck_info_index)
    .bind(&data.attack_deck_item_info_index)
    .bind(&data.defense_deck_info_index)
    .bind(&data.defense_deck_item_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PvpBattleDeckInfo>> {
    sqlx::query_as::<_, PvpBattleDeckInfo>("SELECT * FROM PvpBattleDeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PvpBattleDeckInfo rows for a UID.
pub async fn delete_pvp_battle_deck_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleDeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}