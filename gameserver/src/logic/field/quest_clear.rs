use crate::logic::game::equip::{create_new_equip, get_equip_with_base, to_dbinfo};
use anyhow::{Result, anyhow};
use bd2::proto::proto_net::{
    CharDbInfo, CostumeDbInfo, EquipDbInfo, ItemDbInfo, QuestClearResponse, QuestDbInfo,
    RewardDbInfoBundle,
};
use data::exceldb;
use database::db::char::char_info::{get_char_info, insert as insert_char};
use database::db::costume::costume_info::{get_costume_info, insert as insert_costume};
use database::models::game::char::char_info::CharInfo;
use database::models::game::costume::costume_info::CostumeInfo;
use sqlx::SqlitePool;

/// Reward item types that need a real, playable DB row instead of a generic ItemInfo stack --
/// matches the reference server's GameQuestService.GetRewardDbInfoBundle special-casing.
const ITEM_TYPE_EQUIP: i32 = 10;
const ITEM_TYPE_CHAR: i32 = 6;
const ITEM_TYPE_COSTUME: i32 = 11;

#[derive(Debug, Clone, PartialEq)]
pub struct QuestReward {
    pub item_id: i32,
    pub item_type: i32,
    pub count: i32,
}

pub async fn handle_quest_clear(
    pool: &SqlitePool,
    uid: i64,
    quest_id: i32,
) -> Result<QuestClearResponse> {
    let game_data = exceldb::get();

    // Step 1: Find quest entry. Same real content gap as quest_update (a captured-data hole,
    // not a code bug -- some quests, e.g. quest 2 in this pack, have no row in QuestTable1 at
    // all). Hard-failing here used to surface as a client-side error popup (10404) that
    // silently blocks story progress -- e.g. entering the chief's house and having nothing
    // happen, confirmed live (2026-09-30). Match the established convention: record the clear
    // and let the player continue, just without rewards/next-quest data we have no way to
    // determine from missing captured data.
    let Some(quest) = game_data.questtable1.get(quest_id) else {
        tracing::warn!(
            "QuestClear: quest_id={} not in captured QuestTable1 (data gap) -- marking cleared anyway, no rewards/next_quest available.",
            quest_id
        );
        mark_quest_complete(pool, uid, quest_id).await?;
        // CORRECTION (2026-09-30): `quest_info: None` here crashed the CLIENT with a
        // NullReferenceException inside TimelinePlayManager's QuestClearResponse handler --
        // confirmed live via the client's own crash report (sent to /sendmail as a side effect,
        // decoded from the server access log). The client unconditionally reads into QuestInfo
        // after a clear; it never expects that field to be absent. A real response always
        // carries a quest_info (the real next quest), so a null Option here is itself an
        // unrepresentable state, not just "no data" -- give it a best-guess placeholder
        // (quest_id + 1, the game's own convention for this pack's linear quest numbering)
        // instead of a real gap. Real content gap in the data, but the RESPONSE must still be a
        // valid, populated message.
        return Ok(QuestClearResponse {
            reward_info_bundle: Some(RewardDbInfoBundle::default()),
            quest_info: Some(QuestDbInfo {
                id: Some(quest_id + 1),
                value: Some(0),
                object_id: vec![],
                quest_level: Some(0),
                quest_opt: Some(0),
            }),
            clear_quest_id: Some(quest_id),
            ..Default::default()
        });
    };

    // Step 2: Mark quest complete in DB
    mark_quest_complete(pool, uid, quest_id).await?;

    // Step 3: Update clear info (quest level and max clear)
    upsert_quest_level_info(pool, uid, quest_id).await?;
    upsert_quest_max_clear_info(pool, uid, quest_id).await?;

    // Step 4: Roll rewards from quest definition
    let rewards = extract_rewards(quest)?;

    // Step 5: Grant rewards -- Equip/Costume/Char rewards get real, usable DB rows (matching the
    // reference server's GameQuestService.GetRewardDbInfoBundle); everything else is a plain
    // ItemInfo stack as before.
    let granted = grant_rewards(pool, uid, &rewards).await?;

    // Step 6: Build RewardDBInfoBundle
    let reward_bundle = RewardDbInfoBundle {
        item_info: granted.item_info.clone(),
        view_item_info: granted.item_info.clone(),
        original_item_info: granted.item_info,
        char_info: granted.char_info,
        costume_info: granted.costume_info,
        equip_info: granted.equip_info,
        my_room_trophy_info: vec![],
        item_auto_exchange_info: granted.item_auto_exchange_info,
        item_auto_upgrade_info: granted.item_auto_upgrade_info,
        repaid_currency: vec![],
    };

    // Step 7: Build next quest info
    let quest_info = quest.next_quest_id.map(|id| QuestDbInfo {
        id: Some(id),
        ..Default::default()
    });

    // Step 8: Build final response
    Ok(QuestClearResponse {
        reward_info_bundle: Some(reward_bundle),
        quest_info,
        clear_quest_id: Some(quest_id),
        ..Default::default()
    })
}

// -------------------- DB Operations --------------------

async fn mark_quest_complete(pool: &SqlitePool, uid: i64, quest_id: i32) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE UserQuest
        SET Status = 3, RewardClaimed = 1
        WHERE Uid = ? AND QuestId = ?
        "#,
    )
    .bind(uid)
    .bind(quest_id)
    .execute(pool)
    .await?;
    Ok(())
}

async fn upsert_quest_level_info(pool: &SqlitePool, uid: i64, clear_quest: i32) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO QuestLevelInfo (Uid, PackId, QuestLevel, ClearQuest, QuestOpt, IsLevelComplete)
        VALUES (?, 1, 0, ?, 0, 0)
        ON CONFLICT(Uid, PackId)
        DO UPDATE SET ClearQuest = excluded.ClearQuest
        "#,
    )
    .bind(uid)
    .bind(clear_quest)
    .execute(pool)
    .await?;
    Ok(())
}

async fn upsert_quest_max_clear_info(pool: &SqlitePool, uid: i64, clear_quest: i32) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO QuestMaxClearInfo (Uid, PackId, MaxClearId)
        VALUES (?, 1, ?)
        ON CONFLICT(Uid, PackId)
        DO UPDATE SET MaxClearId = excluded.MaxClearId
        "#,
    )
    .bind(uid)
    .bind(clear_quest)
    .execute(pool)
    .await?;
    Ok(())
}

// -------------------- Reward Extraction --------------------

fn extract_rewards(quest: &data::exceldb::questtable1::Questtable1) -> Result<Vec<QuestReward>> {
    let types = quest
        .reward_type
        .as_ref()
        .ok_or_else(|| anyhow!("Quest missing reward_type"))?;
    let ids = quest
        .reward_id
        .as_ref()
        .ok_or_else(|| anyhow!("Quest missing reward_id"))?;
    let counts = quest
        .reward_count
        .as_ref()
        .ok_or_else(|| anyhow!("Quest missing reward_count"))?;

    let len = types.len().min(ids.len()).min(counts.len());
    Ok((0..len)
        .map(|i| QuestReward {
            item_id: ids[i],
            item_type: types[i],
            count: counts[i],
        })
        .collect())
}

// -------------------- Reward Granting --------------------

#[derive(Default)]
struct GrantedRewards {
    item_info: Vec<ItemDbInfo>,
    char_info: Vec<CharDbInfo>,
    costume_info: Vec<CostumeDbInfo>,
    equip_info: Vec<EquipDbInfo>,
    item_auto_upgrade_info: Vec<bd2::proto::proto_net::ItemAutoUpgradeInfo>,
    item_auto_exchange_info: Vec<bd2::proto::proto_net::ItemAutoExchangeInfo>,
}

fn char_db_info(row: &CharInfo) -> CharDbInfo {
    CharDbInfo {
        inven_index: row.inven_index,
        id: row.id,
        hp: row.hp,
        level: row.level,
        costume_id: row.costume_id,
        exp: row.exp,
        use_costume: row.use_costume,
        talent_level: row.talent_level,
        talent_exp: row.talent_exp,
        solidarity_reward: row.solidarity_reward,
        expiry_time: row.expiry_time,
        pictorialbook_info: vec![],
        connect_potential_costume: row.connect_potential_costume,
    }
}

fn costume_db_info(row: &CostumeInfo) -> CostumeDbInfo {
    CostumeDbInfo {
        inven_index: row.inven_index,
        id: row.id,
        level: row.level,
        use_char: row.use_char,
        pictorialbook_info: vec![],
        sort_id: row.sort_id,
        use_my_room_count: row.use_my_room_count,
        potential_id: vec![],
        design_id: row.design_id,
    }
}

/// Grants one Costume-type reward, mirroring the reference server's `GetRewardDbInfoBundle`
/// Costume case: if the account doesn't yet own the underlying character, both the character
/// and the costume are created together (the costume becomes the character's equipped
/// costume); if the character is owned but not this specific costume, only the costume is
/// created (left unequipped, exactly as the reference does); if the costume is already owned
/// and below max level, it's leveled up instead of duplicated.
async fn grant_costume_reward(
    pool: &SqlitePool,
    uid: i64,
    costume_id: i32,
    granted: &mut GrantedRewards,
) -> Result<()> {
    let game_data = exceldb::get();
    let Some(costume_def) = game_data.costumetable.get(costume_id) else {
        tracing::warn!("quest_clear: CostumeTable missing id {}", costume_id);
        return Ok(());
    };
    let unique_char_id = costume_def.use_unique_char_id;
    let max_level = costume_def.max_level;
    let skill_group_id = costume_def.skill_group_id;

    let owned_chars = get_char_info(pool, uid).await.unwrap_or_default();
    let owned_char = owned_chars.iter().find(|c| {
        c.id.and_then(|id| game_data.chartable.get(id))
            .map(|ch| ch.unique_char_id == unique_char_id)
            .unwrap_or(false)
    });

    let now = chrono::Utc::now().timestamp_millis();

    if owned_char.is_none() {
        // Character not owned yet: grant character + costume together.
        let Some(char_def) = game_data
            .chartable
            .iter()
            .find(|ch| ch.unique_char_id == unique_char_id)
        else {
            tracing::warn!(
                "quest_clear: no CharTable row for unique_char_id {} (costume {})",
                unique_char_id,
                costume_id
            );
            return Ok(());
        };

        let char_inven_index = now;
        let costume_inven_index = -char_inven_index;
        let char_row = CharInfo {
            index: 0,
            uid,
            inven_index: Some(char_inven_index),
            id: Some(char_def.id),
            hp: Some(char_def.health_value.round() as i64),
            level: Some(1),
            costume_id: Some(costume_id),
            exp: Some(0),
            use_costume: Some(costume_inven_index),
            talent_level: Some(1),
            talent_exp: Some(0),
            solidarity_reward: Some(1),
            expiry_time: Some(-32400000),
            pictorialbook_info_index: None,
            connect_potential_costume: skill_group_id,
            class_level: 0,
        };
        insert_char(pool, &char_row).await?;
        let costume_row = CostumeInfo {
            index: 0,
            uid,
            inven_index: Some(costume_inven_index),
            id: Some(costume_id),
            level: Some(1),
            use_char: Some(char_inven_index),
            pictorialbook_info_index: None,
            sort_id: None,
            use_my_room_count: None,
            potential_id: None,
            design_id: None,
        };
        insert_costume(pool, &costume_row).await?;

        granted.char_info.push(char_db_info(&char_row));
        granted.costume_info.push(costume_db_info(&costume_row));
        granted.item_info.push(ItemDbInfo {
            id: Some(costume_id),
            r#type: Some(ITEM_TYPE_COSTUME),
            count: Some(1),
            ..Default::default()
        });
        granted.item_info.push(ItemDbInfo {
            id: Some(char_def.id),
            r#type: Some(ITEM_TYPE_CHAR),
            count: Some(1),
            ..Default::default()
        });
        return Ok(());
    }

    // Character already owned -- check whether this specific costume is too.
    let owned_costumes = get_costume_info(pool, uid).await.unwrap_or_default();
    if let Some(existing_costume) = owned_costumes.iter().find(|c| c.id == Some(costume_id)) {
        let current_level = existing_costume.level.unwrap_or(1);
        if max_level.map(|m| current_level < m).unwrap_or(false) {
            // Below max level: level the existing costume up instead of duplicating it.
            let new_level = current_level + 1;
            database::db::costume::costume_info::set_level(
                pool,
                uid,
                existing_costume.index,
                new_level,
            )
            .await?;
            granted
                .item_auto_upgrade_info
                .push(bd2::proto::proto_net::ItemAutoUpgradeInfo {
                    inven_index: existing_costume.inven_index,
                    item_type: Some(ITEM_TYPE_COSTUME),
                    item_id: Some(costume_id),
                    before_level: Some(current_level),
                    after_level: Some(new_level),
                    sort_id: None,
                });
            granted.item_info.push(ItemDbInfo {
                id: Some(costume_id),
                r#type: Some(ITEM_TYPE_COSTUME),
                count: Some(1),
                ..Default::default()
            });
            granted.item_info.push(ItemDbInfo {
                id: Some(unique_char_id),
                r#type: Some(ITEM_TYPE_CHAR),
                count: Some(1),
                ..Default::default()
            });
        } else {
            // Already at max level: converts into a generic duplicate-reward item instead
            // (matches the reference server's dupe-exchange behavior for maxed costumes).
            let exchange_item = ItemDbInfo {
                r#type: Some(20),
                count: Some(1),
                ..Default::default()
            };
            granted.item_info.push(exchange_item);
            granted
                .item_auto_exchange_info
                .push(bd2::proto::proto_net::ItemAutoExchangeInfo {
                    original_item_type: Some(ITEM_TYPE_COSTUME),
                    original_item_id: Some(costume_id),
                    original_item_count: Some(1),
                    exchange_item_type: Some(20),
                    exchange_item_id: None,
                    exchange_item_count: Some(1),
                    sort_id: None,
                });
        }
        return Ok(());
    }

    // Character owned, but not this costume yet: grant just the costume (unequipped).
    let costume_inven_index = now;
    let costume_row = CostumeInfo {
        index: 0,
        uid,
        inven_index: Some(costume_inven_index),
        id: Some(costume_id),
        level: Some(1),
        use_char: None,
        pictorialbook_info_index: None,
        sort_id: None,
        use_my_room_count: None,
        potential_id: None,
        design_id: None,
    };
    insert_costume(pool, &costume_row).await?;
    granted.costume_info.push(costume_db_info(&costume_row));
    granted.item_info.push(ItemDbInfo {
        id: Some(costume_id),
        r#type: Some(ITEM_TYPE_COSTUME),
        count: Some(1),
        ..Default::default()
    });
    granted.item_info.push(ItemDbInfo {
        id: Some(unique_char_id),
        r#type: Some(ITEM_TYPE_CHAR),
        count: Some(1),
        ..Default::default()
    });
    Ok(())
}

async fn grant_rewards(
    pool: &SqlitePool,
    uid: i64,
    items: &[QuestReward],
) -> Result<GrantedRewards> {
    let mut granted = GrantedRewards::default();
    let mut generic_items = Vec::new();

    for item in items {
        match item.item_type {
            ITEM_TYPE_EQUIP => match create_new_equip(pool, uid, item.item_id).await {
                Ok(equip_index) => {
                    if let Some((equip, base)) = get_equip_with_base(pool, uid, equip_index).await
                    {
                        granted.equip_info.push(to_dbinfo(&equip, base.as_ref()));
                    }
                    granted.item_info.push(ItemDbInfo {
                        id: Some(item.item_id),
                        r#type: Some(ITEM_TYPE_EQUIP),
                        count: Some(1),
                        ..Default::default()
                    });
                }
                Err(e) => {
                    tracing::warn!(
                        "quest_clear: failed to grant equip {}: {:?}",
                        item.item_id,
                        e
                    );
                }
            },
            ITEM_TYPE_COSTUME => {
                if let Err(e) = grant_costume_reward(pool, uid, item.item_id, &mut granted).await {
                    tracing::warn!(
                        "quest_clear: failed to grant costume {}: {:?}",
                        item.item_id,
                        e
                    );
                }
            }
            _ => generic_items.push(item.clone()),
        }
    }

    let plain_items = add_items_to_inventory(pool, uid, &generic_items).await?;
    granted.item_info.extend(plain_items);
    Ok(granted)
}

// -------------------- Inventory --------------------

async fn add_items_to_inventory(
    pool: &SqlitePool,
    uid: i64,
    items: &[QuestReward],
) -> Result<Vec<ItemDbInfo>> {
    use sqlx::{Row, query};

    let now = chrono::Utc::now().timestamp_millis();
    let row = query("SELECT COALESCE(MAX(InvenIndex), 0) as idx FROM ItemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    let mut idx: i64 = row.get("idx");

    let mut results = Vec::new();

    for item in items {
        idx += 1;
        query(
            "INSERT INTO ItemInfo (Uid, InvenIndex, Id, Type, Count, KeepFlag, TimeValue, SortId, UseCount)
             VALUES (?, ?, ?, ?, ?, 0, ?, 0, 0)",
        )
        .bind(uid)
        .bind(idx)
        .bind(item.item_id)
        .bind(item.item_type)
        .bind(item.count)
        .bind(now)
        .execute(pool)
        .await?;

        results.push(ItemDbInfo {
            inven_index: Some(idx),
            id: Some(item.item_id),
            r#type: Some(item.item_type),
            count: Some(item.count),
            keep_flag: Some(0),
            time_value: Some(now),
            pictorialbook_info: None,
            expiry_time: None,
            sort_id: Some(0),
            use_count: Some(0),
        });
    }

    Ok(results)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn setup() {
        use std::sync::Once;
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            let data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("data")
                .join("tables");

            exceldb::init(data_path.to_str().unwrap()).expect("Failed to initialize game data");
        });
    }
}
