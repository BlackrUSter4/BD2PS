use anyhow::{Result, anyhow};
use bd2::proto::proto_net::{ItemDbInfo, QuestClearResponse, QuestDbInfo, RewardDbInfoBundle};
use data::exceldb;
use sqlx::SqlitePool;

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

    // Step 1: Find quest entry
    let quest = game_data
        .questtable1
        .get(quest_id)
        .ok_or_else(|| anyhow!("QuestTable1: quest {} not found", quest_id))?;

    // Step 2: Mark quest complete in DB
    mark_quest_complete(pool, uid, quest_id).await?;

    // Step 3: Update clear info (quest level and max clear)
    upsert_quest_level_info(pool, uid, quest_id).await?;
    upsert_quest_max_clear_info(pool, uid, quest_id).await?;

    // Step 4: Roll rewards from quest definition
    let rewards = extract_rewards(quest)?;

    // Step 5: Insert rewards into inventory
    let item_db_infos = add_items_to_inventory(pool, uid, &rewards).await?;

    // Step 6: Build RewardDBInfoBundle
    let reward_bundle = RewardDbInfoBundle {
        item_info: item_db_infos.clone(),
        view_item_info: item_db_infos.clone(),
        original_item_info: item_db_infos.clone(),
        char_info: vec![],
        costume_info: vec![],
        equip_info: vec![],
        my_room_trophy_info: vec![],
        item_auto_exchange_info: vec![],
        item_auto_upgrade_info: vec![],
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
