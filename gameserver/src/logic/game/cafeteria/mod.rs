pub mod cafeteria_cumulative_reward;
pub mod cafeteria_daily_connection_costume_refresh;
pub mod cafeteria_event_npc_interaction_reward;
pub mod cafeteria_info;
pub mod cafeteria_introduction_story_reward;
pub mod cafeteria_level_up;
pub mod cafeteria_manage_item_add;
pub mod cafeteria_rare_npc_interaction_reward;
pub mod cafeteria_regular_costume_interaction_all_reward;
pub mod cafeteria_regular_costume_interaction_reward;
pub mod cafeteria_regular_costume_note_all_reward;
pub mod cafeteria_regular_costume_note_info;
pub mod cafeteria_regular_costume_note_reward;
pub mod cafeteria_reward_receipt_time_update_using_cheat;
pub mod cafeteria_spawn_reset;
pub mod cafeteria_spawn_reset_cheat;

use bd2::proto::proto_net::Notify;

pub fn default_notify() -> Notify {
    Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    }
}

/// Real gold, per the convention established in the Friendship/SpineInteraction rounds:
/// no table anywhere names an explicit currency item id, but item id 4 / type 1 is confirmed
/// real gold (`CurrencyTable` id 4).
pub const GOLD_ITEM_ID: i32 = 4;
pub const GOLD_ITEM_TYPE: i32 = 1;

/// No level-up cost/reward table exists anywhere for Cafeteria (`CafeteriaLevelTable` only
/// carries per-level unlock config: costume slot cap, npc spawn count, facility skill values —
/// nothing resembling a currency cost or a reward amount). Leveling up is therefore free, and
/// the "cumulative reward" it grants is a flat placeholder gold amount.
pub const LEVEL_UP_REWARD_GOLD: i32 = 100;

/// `CafeteriaUniqueNpcSpawnTable` (rare NPCs) has spawn-timing config only, no reward fields at
/// all — flat placeholder gold per rare-NPC interaction.
pub const RARE_NPC_REWARD_GOLD: i32 = 50;

/// No income-rate table exists for the passive "cumulative reward" (idle income) mechanic —
/// `CafeteriaDefaultTable.min_reward_time`/`max_reward_time` bound how many minutes of idle time
/// can accrue, but nothing states gold-per-minute. Placeholder rate.
pub const CUMULATIVE_REWARD_GOLD_PER_MINUTE: i32 = 1;

/// `CafeteriaManageTable`'s 121 rows have no explicit ordering field beyond a (groupId, id) pair
/// that resets per group — the captured JSON array's own order is used as the unlock sequence,
/// walked one row at a time via `CafeteriaInfo.ongoing_manage_id` as a plain 0-based index.
pub fn manage_table_len() -> usize {
    data::exceldb::get().cafeteriamanagetable.all().len()
}

/// `DailyRegularCostumeIds`/`RewardedDailyRegularCostumeIds` are stored as JSON-array TEXT
/// (see migration 395) since the underlying proto fields are `repeated int32`.
pub fn to_id_list(stored: &Option<String>) -> Vec<i32> {
    stored
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default()
}

pub fn from_id_list(ids: &[i32]) -> Option<String> {
    serde_json::to_string(ids).ok()
}

/// Shared by `CafeteriaSpawnReset` and `CafeteriaSpawnResetCheat` (identical response shape and
/// mechanic in this project — no real cooldown gating exists to distinguish "cheat bypasses the
/// cooldown" from the normal path, since no such cooldown gating was implemented in the first
/// place, so both simply re-roll).
pub async fn perform_spawn_reset(
    pool: &sqlx::SqlitePool,
    uid: i64,
) -> anyhow::Result<database::models::game::cafeteria::cafeteria_info::CafeteriaInfo> {
    use database::db::cafeteria::cafeteria_info;
    use rand::seq::IndexedRandom;

    let mut info_row = cafeteria_info::get_or_create(pool, uid).await?;

    let level = info_row.level.unwrap_or(1);
    let costume_max = data::exceldb::get()
        .cafeterialeveltable
        .get(level)
        .map(|l| l.cafeteria_costume_max)
        .unwrap_or(5) as usize;

    let mut costume_ids: Vec<i32> = data::exceldb::get()
        .cafeteriacostumetable
        .all()
        .iter()
        .map(|c| c.costume_id)
        .collect();
    costume_ids.sort_unstable();
    costume_ids.dedup();

    let mut rng = rand::thread_rng();
    let chosen: Vec<i32> = costume_ids
        .choose_multiple(&mut rng, costume_max.min(costume_ids.len()))
        .copied()
        .collect();
    let connection_costume = chosen.choose(&mut rng).copied();

    info_row.daily_regular_costume_ids = from_id_list(&chosen);
    info_row.rewarded_daily_regular_costume_ids = from_id_list(&[]);
    info_row.daily_connection_costume_id = connection_costume;
    info_row.can_get_phone_number = Some(0);
    info_row.daily_npc_reward_currency_count = Some(0);

    cafeteria_info::update_cafeteria_info(pool, &info_row).await?;
    Ok(info_row)
}
