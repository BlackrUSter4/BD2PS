use anyhow::{Result, anyhow};
use bd2::proto::proto_net::{
    FieldObjectRewardRequest, FieldObjectRewardResponse, ItemDbInfo, MonsterDbInfo,
    RewardDbInfoBundle,
};
use data::exceldb;
use sqlx::SqlitePool;

#[derive(Debug, Clone, PartialEq)]
struct ItemDrop {
    id: i32,
    item_type: i32,
    count: i32,
}

pub async fn collect_field_object(
    pool: &SqlitePool,
    user_id: i64,
    request: FieldObjectRewardRequest,
) -> Result<FieldObjectRewardResponse> {
    let game_data = exceldb::get();

    let pack_id = request.pack_id.ok_or_else(|| anyhow!("Missing pack_id"))?;
    let field_object_id = request
        .field_object_id
        .ok_or_else(|| anyhow!("Missing field_object_id"))?;
    let field_object_group_id = request
        .field_object_group_id
        .ok_or_else(|| anyhow!("Missing field_object_group_id"))?;

    // Step 1: Validate the field object exists (pack-scoped: `id` collides
    // across packs, see Fieldrewardobjecttable::pack_id's doc comment).
    let field_object = game_data
        .fieldrewardobjecttable
        .get_by_pack(pack_id, field_object_id)
        .ok_or_else(|| anyhow!("Field object {} not found in pack {}", field_object_id, pack_id))?;

    if field_object.field_object_group_id != field_object_group_id {
        return Err(anyhow!(
            "Field object {} does not belong to group {}",
            field_object_id,
            field_object_group_id
        ));
    }

    // Step 2: Get the group configuration (also pack-scoped)
    let object_group = game_data
        .fieldrewardobjectgrouptable
        .get_by_pack(pack_id, field_object_group_id)
        .ok_or_else(|| anyhow!("Field object group {} not found in pack {}", field_object_group_id, pack_id))?;

    // Step 3: Get the reward group configuration
    let reward_group_id = object_group.reward_group_id.ok_or_else(|| {
        anyhow!(
            "No reward group configured for group {}",
            field_object_group_id
        )
    })?;

    let reward_group = game_data
        .rewardgrouptable
        .get(reward_group_id)
        .ok_or_else(|| anyhow!("Reward group {} not found", reward_group_id))?;

    // Step 4: Roll for rewards
    let original_items = roll_rewards(&reward_group)?;

    // Step 5: Stack identical items
    let stacked_items = stack_items(original_items.clone());

    // Step 6: Add items to user's inventory and get the DB records
    let item_db_infos = add_items_to_inventory(pool, user_id, &stacked_items).await?;

    // Step 7: Create view_item_info (simplified view without DB fields)
    let view_item_infos: Vec<ItemDbInfo> = stacked_items
        .iter()
        .map(|item| ItemDbInfo {
            id: Some(item.id),
            r#type: Some(item.item_type),
            count: Some(item.count),
            ..Default::default()
        })
        .collect();

    // Step 8: Create original_item_info (before stacking)
    let original_item_infos: Vec<ItemDbInfo> = original_items
        .iter()
        .map(|item| ItemDbInfo {
            id: Some(item.id),
            r#type: Some(item.item_type),
            count: Some(item.count),
            ..Default::default()
        })
        .collect();

    // Step 9: Build protobuf response
    let reward_bundle = RewardDbInfoBundle {
        item_info: item_db_infos,
        view_item_info: view_item_infos,
        original_item_info: original_item_infos,
        ..Default::default()
    };

    let response = FieldObjectRewardResponse {
        reward_info_bundle: Some(reward_bundle),
        monster_info: Some(MonsterDbInfo {
            active_flag: Some(true), // ← Hardcoded for now
            ..Default::default()
        }),
        field_buff_info: vec![],
        char_info: vec![],
    };

    Ok(response)
}

fn roll_rewards(
    reward_group: &data::exceldb::rewardgrouptable::Rewardgrouptable,
) -> Result<Vec<ItemDrop>> {
    let drop_count = reward_group
        .drop_count
        .ok_or_else(|| anyhow!("No drop count configured"))?;

    let item_ids = reward_group
        .item_id
        .as_ref()
        .ok_or_else(|| anyhow!("No item IDs configured"))?;
    let item_types = reward_group
        .item_type
        .as_ref()
        .ok_or_else(|| anyhow!("No item types configured"))?;
    let item_counts = reward_group
        .item_count
        .as_ref()
        .ok_or_else(|| anyhow!("No item counts configured"))?;
    let ratios = reward_group
        .ratio
        .as_ref()
        .ok_or_else(|| anyhow!("No ratios configured"))?;

    let mut results = Vec::new();

    for _ in 0..drop_count {
        let item = roll_weighted_random(item_ids, item_types, item_counts, ratios);
        results.push(item);
    }

    Ok(results)
}

fn roll_weighted_random(
    item_ids: &[i32],
    item_types: &[i32],
    item_counts: &[i32],
    ratios: &[i32],
) -> ItemDrop {
    use rand::Rng;

    let total_weight: i32 = ratios.iter().sum();
    let roll = rand::thread_rng().gen_range(0..total_weight);

    let mut cumulative = 0;
    for (idx, &ratio) in ratios.iter().enumerate() {
        cumulative += ratio;
        if roll < cumulative {
            return ItemDrop {
                id: item_ids[idx],
                item_type: item_types[idx],
                count: item_counts[idx],
            };
        }
    }

    ItemDrop {
        id: item_ids[0],
        item_type: item_types[0],
        count: item_counts[0],
    }
}

fn stack_items(items: Vec<ItemDrop>) -> Vec<ItemDrop> {
    let mut stacked = std::collections::HashMap::new();

    for item in items {
        let key = (item.id, item.item_type);
        stacked
            .entry(key)
            .and_modify(|count| *count += item.count)
            .or_insert(item.count);
    }

    stacked
        .into_iter()
        .map(|((id, item_type), count)| ItemDrop {
            id,
            item_type,
            count,
        })
        .collect()
}

/// Insert items into inventory and return the ItemDBInfo records
async fn add_items_to_inventory(
    pool: &SqlitePool,
    user_id: i64,
    items: &[ItemDrop],
) -> Result<Vec<ItemDbInfo>> {
    use sqlx::{Row, query};

    let mut item_db_infos = Vec::new();
    let now = chrono::Utc::now().timestamp_millis();

    // Get the last InvenIndex for this user
    let last_inven_index_row = query(
        "SELECT COALESCE(MAX(InvenIndex), 0) as max_index
         FROM ItemInfo
         WHERE Uid = ?",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let mut current_inven_index: i64 = last_inven_index_row.get("max_index");

    for item in items {
        // Increment the inventory index
        current_inven_index += 1;

        // Insert new item
        query(
            "INSERT INTO ItemInfo (Uid, InvenIndex, Id, Type, Count, KeepFlag, TimeValue, SortId, UseCount)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(user_id)
        .bind(current_inven_index)
        .bind(item.id)
        .bind(item.item_type)
        .bind(item.count)
        .bind(0) // KeepFlag
        .bind(now) // TimeValue
        .bind(0) // SortId
        .bind(0) // UseCount
        .execute(pool)
        .await?;

        // Create the ItemDBInfo representing the database record
        item_db_infos.push(ItemDbInfo {
            inven_index: Some(current_inven_index),
            id: Some(item.id),
            r#type: Some(item.item_type),
            count: Some(item.count),
            time_value: Some(now),
            ..Default::default()
        });
    }

    Ok(item_db_infos)
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

            exceldb::init(data_path.to_str().unwrap())
                .expect("Failed to initialize game data for tests");
        });
    }

    #[test]
    fn test_roll_rewards() {
        setup();
        let game_data = exceldb::get();

        let reward_group = game_data
            .rewardgrouptable
            .get(110001)
            .expect("Reward group should exist");

        let items = roll_rewards(&reward_group).expect("Should roll rewards");

        assert_eq!(items.len(), 2);
        println!("\nRolled rewards:");
        for item in &items {
            println!(
                "  Item {} x{} (type {})",
                item.id, item.count, item.item_type
            );
        }
    }

    #[test]
    fn test_item_stacking() {
        let items = vec![
            ItemDrop {
                id: 1014,
                item_type: 5,
                count: 16,
            },
            ItemDrop {
                id: 1014,
                item_type: 5,
                count: 16,
            },
        ];

        let stacked = stack_items(items);

        assert_eq!(stacked.len(), 1);
        assert_eq!(stacked[0].count, 32);
    }

    #[test]
    fn test_weighted_random_distribution() {
        let item_ids = vec![1013, 1014, 1004, 1010];
        let item_types = vec![5, 5, 5, 5];
        let item_counts = vec![16, 16, 8, 8];
        let ratios = vec![65, 29, 5, 1];

        let mut results = std::collections::HashMap::new();
        for _ in 0..10_000 {
            let item = roll_weighted_random(&item_ids, &item_types, &item_counts, &ratios);
            *results.entry(item.id).or_insert(0) += 1;
        }

        let item_1013_pct = (*results.get(&1013).unwrap_or(&0) as f32 / 10_000.0) * 100.0;
        assert!(item_1013_pct > 60.0 && item_1013_pct < 70.0);
    }
}
