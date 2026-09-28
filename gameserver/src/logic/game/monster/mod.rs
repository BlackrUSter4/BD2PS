use bd2::proto::proto_net::Notify;

pub const GOLD_ITEM_ID: i32 = 4;
pub const GOLD_ITEM_TYPE: i32 = 1;

/// The one real MonsterHuntTable boss (id 72 as of the currently captured client data) is
/// treated as the always-active season boss. If more bosses get captured later, this should
/// pick whichever is scheduled instead of always the first.
pub fn default_boss_id() -> i32 {
    data::exceldb::get()
        .monsterhunttable
        .iter()
        .next()
        .map(|b| b.id)
        .unwrap_or(72)
}

/// Boss HP at a given level, from MonsterHuntTable's real level-scaling formula
/// (`levelUpHealthRate * levelUpHealthSlope^(level-1)`). This IS real data-driven scaling,
/// not a placeholder — only the per-battle damage roll (in monster_hunt_quick_battle.rs) is
/// a placeholder, since no formula/table anywhere states how much damage one attempt deals.
pub fn boss_hp_at_level(boss_id: i32, level: i32) -> i64 {
    let table = &data::exceldb::get().monsterhunttable;
    match table.get(boss_id) {
        Some(b) => {
            let slope = (b.level_up_health_slope as f64).max(1.0);
            let hp = b.level_up_health_rate as f64 * slope.powi((level - 1).max(0));
            hp.round() as i64
        }
        None => 33000,
    }
}

/// Real cross-account rank + percentile for this account, computed fresh from every
/// account's MonsterHuntUserInfo row (same non-bot-padded approach as Colosseum/PvP/Guild).
pub async fn compute_rank(pool: &sqlx::SqlitePool, uid: i64) -> (i32, f64) {
    let all = database::db::monster::monster_hunt_user_info::rank_all(pool, 100000)
        .await
        .unwrap_or_default();
    let total = all.len().max(1);
    match all.iter().position(|r| r.uid == uid) {
        Some(pos) => (
            (pos + 1) as i32,
            ((pos + 1) as f64 / total as f64) * 100.0,
        ),
        None => (total as i32, 100.0),
    }
}

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

pub mod monster_hunt_change_team;
pub mod monster_hunt_deck_info;
pub mod monster_hunt_deck_save;
pub mod monster_hunt_preset_delete;
pub mod monster_hunt_preset_info;
pub mod monster_hunt_preset_info_change;
pub mod monster_hunt_preset_save;
pub mod monster_hunt_preset_slot_add;
pub mod monster_hunt_preset_use;
pub mod monster_hunt_quick_battle;
pub mod monster_hunt_rank_info;
pub mod monster_hunt_schedule_info;
pub mod monster_hunt_season_reward;
pub mod monster_hunt_user_info;
pub mod monster_info;
