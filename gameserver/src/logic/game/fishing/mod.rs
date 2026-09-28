pub mod fishing_bait_use;
pub mod fishing_bite_end;
pub mod fishing_bite_fish_hp_update;
pub mod fishing_bite_fish_stamina_update;
pub mod fishing_bite_start;
pub mod fishing_bite_start_cheat;
pub mod fishing_boat_skin_buy;
pub mod fishing_boat_skin_set;
pub mod fishing_boat_upgrade;
pub mod fishing_casting;
pub mod fishing_collection_info;
pub mod fishing_fish_auto;
pub mod fishing_fish_inven_slot_add;
pub mod fishing_fish_lock;
pub mod fishing_info;
pub mod fishing_item_info;
pub mod fishing_map_buy;
pub mod fishing_multi_room_info;
pub mod fishing_rod_set;
pub mod fishing_shop_buy;
pub mod fishing_shop_buy_info;
pub mod fishing_shop_sell;
pub mod fishing_trap_cumulative_reward;
pub mod fishing_trap_info;
pub mod fishing_user_info;
pub mod fishing_voyage_end;
pub mod fishing_voyage_start;

use bd2::proto::proto_net::{
    FishingCollectionDbInfo, FishingDbInfo, FishingFishDbInfo, FishingRodDbInfo, ItemDbInfo,
};
use database::models::game::fishing::fishing_collection_info::FishingCollectionInfo;
use database::models::game::fishing::fishing_fish_info::FishingFishInfo;
use database::models::game::fishing::fishing_rod_info::FishingRodInfo;
use database::models::game::fishing::fishing_user_info::FishingUserInfo;

pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

/// No `FishingFishPoolTable`/`FishingFishSizePoolTable`/`FishingGradePoolTable` data was
/// captured from the live client, so there's no real pool to draw a caught fish's id/size
/// from. Deliberate placeholder: a fixed fish id with a size randomized in a plausible range.
/// Flagged throughout every caller of this function.
pub fn placeholder_catch(fish_id_override: Option<i32>) -> (i32, i32) {
    use rand::Rng;
    let fish_id = fish_id_override.unwrap_or(1);
    let size = rand::thread_rng().gen_range(100..=999);
    (fish_id, size)
}

/// Flat placeholder exp-per-catch (no `FishingCharTable`/growth-curve data captured) and a
/// simple threshold level curve (100 exp per level, no cap data available either).
pub const CATCH_EXP: i32 = 10;

pub fn level_for_exp(exp: i32) -> i32 {
    1 + (exp / 100)
}

pub struct ItemDelta {
    pub id: i32,
    pub r#type: i32,
    pub count: i32,
}

pub fn item_deltas(items: &[ItemDbInfo]) -> Vec<ItemDelta> {
    items
        .iter()
        .filter_map(|i| {
            Some(ItemDelta {
                id: i.id?,
                r#type: i.r#type.unwrap_or(0),
                count: i.count.unwrap_or(1),
            })
        })
        .collect()
}

pub async fn try_consume_items(pool: &sqlx::SqlitePool, uid: i64, items: &[ItemDbInfo]) -> bool {
    let deltas = item_deltas(items);
    for d in &deltas {
        match database::db::item::item_info::consume(pool, uid, d.id, d.count).await {
            Ok(true) => {}
            _ => return false,
        }
    }
    true
}

pub fn user_to_dbinfo(user: &FishingUserInfo) -> FishingDbInfo {
    FishingDbInfo {
        exp: Some(user.exp),
        level: Some(user.level),
        boat_level: Some(user.boat_level),
        boat_skin_id: user.boat_skin_id,
        use_rod_inven_index: user.use_rod_inven_index,
        multi_ap_reset_time: user.multi_ap_reset_time,
    }
}

pub fn fish_row_to_dbinfo(row: &FishingFishInfo) -> FishingFishDbInfo {
    FishingFishDbInfo {
        inven_index: Some(row.inven_index),
        id: Some(row.fish_id),
        size: Some(row.size),
        time_value: row.time_value,
        is_lock: Some(row.is_lock),
    }
}

pub fn rod_row_to_dbinfo(row: &FishingRodInfo) -> FishingRodDbInfo {
    FishingRodDbInfo {
        inven_index: Some(row.inven_index),
        id: Some(row.rod_id),
        time_value: row.time_value,
    }
}

pub fn collection_row_to_dbinfo(row: &FishingCollectionInfo) -> FishingCollectionDbInfo {
    FishingCollectionDbInfo {
        fish_id: Some(row.fish_id),
        max_size: Some(row.max_size),
        min_size: Some(row.min_size),
        create_time: row.create_time,
    }
}
