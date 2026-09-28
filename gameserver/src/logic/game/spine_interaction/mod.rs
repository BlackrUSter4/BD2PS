pub mod spine_interaction_achievement_info;
pub mod spine_interaction_achievement_save;
pub mod spine_interaction_record_data_update;
pub mod spine_interaction_record_delete;
pub mod spine_interaction_record_detail_info;
pub mod spine_interaction_record_info;
pub mod spine_interaction_record_name_update;
pub mod spine_interaction_record_save;
pub mod spine_interaction_reward;
pub mod spine_interaction_reward_info;

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

/// This server has no S3 (the real backend uploads SpineInteractionRecord bytes to S3 and
/// hands back a URL) — instead it stores the bytes itself and serves them back from its own
/// address via the SpineInteractionRecordData GET route.
pub fn record_data_url(inven_index: i64) -> String {
    format!("https://127.0.0.1:8443/SpineInteractionRecordData/{inven_index}")
}

/// No SpineInteractionReward-equivalent reward-table data exists anywhere in the captured master
/// data (no per-(interaction_group_id, group_id, id) reward definition was found), so a claim
/// grants this fixed placeholder amount of gold (item id 4, confirmed real from CurrencyTable)
/// rather than fabricating a lookup that doesn't exist. The claim itself (preventing double-grant)
/// is real; only this value is a placeholder.
pub const PLACEHOLDER_REWARD_ITEM_ID: i32 = 4;
pub const PLACEHOLDER_REWARD_ITEM_TYPE: i32 = 1;
pub const PLACEHOLDER_REWARD_COUNT: i32 = 100;
