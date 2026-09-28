use crate::models::game::evil::evil_castle_rogue_like_info::EvilCastleRogueLikeInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of EvilCastleRogueLikeInfo records for a UID.
pub async fn insert_evil_castle_rogue_like_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data
        .get("evilCastleRogueLikeInfo")
        .and_then(|v| v.as_array())
    {
        Some(a) => a,
        None => {
            eprintln!("insert_evil_castle_rogue_like_info: missing or invalid 'evilCastleRogueLikeInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle single nested EvilCastleRogueLikeStateInfo - extract InvenIndex
        let state_info_index = entry
            .get("stateInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());
        let level = entry
            .get("level")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        // Handle repeated nested EvilCastleRogueLikeFloorInfo - extract InvenIndex values
        let floor_info_index =
            if let Some(nested_arr) = entry.get("floorInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle repeated nested DeckInfo - extract InvenIndex values
        let deck_info_index =
            if let Some(nested_arr) = entry.get("deckInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle repeated nested CharInfo - extract InvenIndex values
        let char_info_index =
            if let Some(nested_arr) = entry.get("charInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle repeated nested CostumeInfo - extract InvenIndex values
        let costume_info_index =
            if let Some(nested_arr) = entry.get("costumeInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle repeated nested RelicInfo - extract InvenIndex values
        let relic_info_index =
            if let Some(nested_arr) = entry.get("relicInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle single nested EvilCastleRogueLikeChoiceInfo - extract InvenIndex
        let choice_info_index = entry
            .get("choiceInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());
        let re_roll = entry
            .get("reRoll")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let group_id = entry
            .get("groupId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let id = entry.get("id").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
        // Handle repeated nested EvilCastleRogueLikeGrowthInfo - extract InvenIndex values
        let growth_info_index =
            if let Some(nested_arr) = entry.get("growthInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle single nested EvilCastleRogueLikeEventInfo - extract InvenIndex
        let event_info_index = entry
            .get("eventInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());
        // Handle single nested EvilCastleRogueLikeShopInfo - extract InvenIndex
        let shop_info_index = entry
            .get("shopInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());
        let battle_level = entry
            .get("battleLevel")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let max_try_level = entry
            .get("maxTryLevel")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let obsidian = entry
            .get("obsidian")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let rogue_like_gold = entry
            .get("rogueLikeGold")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let season = entry
            .get("season")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let regular_season = entry
            .get("regularSeason")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let season_reward = entry
            .get("seasonReward")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let max_reward_level = entry
            .get("maxRewardLevel")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let highest_crystal_damage = entry
            .get("highestCrystalDamage")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        sqlx::query(
            r#"
INSERT INTO EvilCastleRogueLikeInfo (
    Uid,
    StateInfoIndex,
    Level,
    FloorInfoIndex,
    DeckInfoIndex,
    CharInfoIndex,
    CostumeInfoIndex,
    RelicInfoIndex,
    ChoiceInfoIndex,
    ReRoll,
    GroupId,
    Id,
    GrowthInfoIndex,
    EventInfoIndex,
    ShopInfoIndex,
    BattleLevel,
    MaxTryLevel,
    Obsidian,
    RogueLikeGold,
    Season,
    RegularSeason,
    SeasonReward,
    MaxRewardLevel,
    HighestCrystalDamage
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&state_info_index)
        .bind(&level)
        .bind(&floor_info_index)
        .bind(&deck_info_index)
        .bind(&char_info_index)
        .bind(&costume_info_index)
        .bind(&relic_info_index)
        .bind(&choice_info_index)
        .bind(&re_roll)
        .bind(&group_id)
        .bind(&id)
        .bind(&growth_info_index)
        .bind(&event_info_index)
        .bind(&shop_info_index)
        .bind(&battle_level)
        .bind(&max_try_level)
        .bind(&obsidian)
        .bind(&rogue_like_gold)
        .bind(&season)
        .bind(&regular_season)
        .bind(&season_reward)
        .bind(&max_reward_level)
        .bind(&highest_crystal_damage)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EvilCastleRogueLikeInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeInfo (
    Uid,
    StateInfoIndex,
    Level,
    FloorInfoIndex,
    DeckInfoIndex,
    CharInfoIndex,
    CostumeInfoIndex,
    RelicInfoIndex,
    ChoiceInfoIndex,
    ReRoll,
    GroupId,
    Id,
    GrowthInfoIndex,
    EventInfoIndex,
    ShopInfoIndex,
    BattleLevel,
    MaxTryLevel,
    Obsidian,
    RogueLikeGold,
    Season,
    RegularSeason,
    SeasonReward,
    MaxRewardLevel,
    HighestCrystalDamage
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.state_info_index)
    .bind(&data.level)
    .bind(&data.floor_info_index)
    .bind(&data.deck_info_index)
    .bind(&data.char_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.relic_info_index)
    .bind(&data.choice_info_index)
    .bind(&data.re_roll)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.growth_info_index)
    .bind(&data.event_info_index)
    .bind(&data.shop_info_index)
    .bind(&data.battle_level)
    .bind(&data.max_try_level)
    .bind(&data.obsidian)
    .bind(&data.rogue_like_gold)
    .bind(&data.season)
    .bind(&data.regular_season)
    .bind(&data.season_reward)
    .bind(&data.max_reward_level)
    .bind(&data.highest_crystal_damage)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeInfo>(
        "SELECT * FROM EvilCastleRogueLikeInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleRogueLikeInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Fetch the single active-run row for a uid, if any.
pub async fn get_one(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<EvilCastleRogueLikeInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeInfo>(
        "SELECT * FROM EvilCastleRogueLikeInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// One live run row per uid — replace it (delete-then-insert).
pub async fn upsert(pool: &SqlitePool, data: &EvilCastleRogueLikeInfo) -> sqlx::Result<()> {
    delete_evil_castle_rogue_like_info(pool, data.uid).await?;
    add_evil_castle_rogue_like_info(pool, data).await
}
