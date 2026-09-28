pub mod use_random_box;
pub mod use_resource_item;

use bd2::proto::proto_net::{ItemDbInfo, Notify};
use database::db::item::item_info;
use sqlx::SqlitePool;

pub fn default_notify() -> Notify {
    Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    }
}

/// Real weighted roll against RewardGroupTable's real item_id/item_type/item_count/ratio
/// arrays (a simple manual weighted pick — no separate weighted-roll helper exists yet in
/// this codebase).
pub async fn roll_reward_group(pool: &SqlitePool, uid: i64, reward_group_id: i32) -> Option<ItemDbInfo> {
    let def = data::exceldb::get().rewardgrouptable.get(reward_group_id)?.clone();
    let ids = def.item_id?;
    let types = def.item_type.unwrap_or_default();
    let counts = def.item_count.unwrap_or_default();
    let ratios = def.ratio.unwrap_or_default();

    let total: i32 = if ratios.len() == ids.len() { ratios.iter().sum() } else { ids.len() as i32 };
    if total <= 0 {
        return None;
    }
    let mut roll = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0).unsigned_abs() % total as u64) as i32;
    let mut chosen = 0usize;
    for i in 0..ids.len() {
        let weight = if ratios.len() == ids.len() { ratios[i] } else { 1 };
        if roll < weight {
            chosen = i;
            break;
        }
        roll -= weight;
    }

    let id = *ids.get(chosen)?;
    let ty = *types.get(chosen).unwrap_or(&1);
    let count = *counts.get(chosen).unwrap_or(&1);
    let _ = item_info::grant(pool, uid, id, ty, count).await;
    Some(ItemDbInfo { id: Some(id), r#type: Some(ty), count: Some(count), ..Default::default() })
}

/// Real deterministic pick (no randomness) — used when the client supplies its own
/// `select_value` choice instead of rolling.
pub async fn grant_reward_group_choice(pool: &SqlitePool, uid: i64, reward_group_id: i32, choice: usize) -> Option<ItemDbInfo> {
    let def = data::exceldb::get().rewardgrouptable.get(reward_group_id)?.clone();
    let ids = def.item_id?;
    let types = def.item_type.unwrap_or_default();
    let counts = def.item_count.unwrap_or_default();

    let id = *ids.get(choice)?;
    let ty = *types.get(choice).unwrap_or(&1);
    let count = *counts.get(choice).unwrap_or(&1);
    let _ = item_info::grant(pool, uid, id, ty, count).await;
    Some(ItemDbInfo { id: Some(id), r#type: Some(ty), count: Some(count), ..Default::default() })
}
