pub mod field_event_spawn_info;
pub mod field_event_spawn_reward;
pub mod field_event_spawn_start;

use bd2::proto::proto_net::Notify;

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

/// No `FieldEventSpawn*Table` master data exists anywhere in this project (not just
/// uncaptured — this event system has no corresponding exceldb table at all) — reward
/// contents are a flat placeholder, same precedent used throughout this project.
pub const FIELD_EVENT_SPAWN_REWARD_ITEM_ID: i32 = 4;
pub const FIELD_EVENT_SPAWN_REWARD_ITEM_TYPE: i32 = 1;
pub const FIELD_EVENT_SPAWN_REWARD_COUNT: i32 = 30;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CaughtEntry {
    pub spawn_event_id: i32,
    pub monster_group_id: i32,
    pub monster_id: i32,
}

pub fn parse_caught(text: &Option<String>) -> Vec<CaughtEntry> {
    text.as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default()
}

pub fn encode_caught(entries: &[CaughtEntry]) -> Option<String> {
    serde_json::to_string(entries).ok()
}
