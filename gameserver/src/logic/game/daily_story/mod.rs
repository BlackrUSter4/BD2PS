pub mod daily_story_clear;
pub mod daily_story_info;

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

/// No table maps a daily-story id to a real reward — same placeholder-grant precedent used
/// throughout this project wherever no reward table exists.
pub const DAILY_STORY_REWARD_ITEM_ID: i32 = 4;
pub const DAILY_STORY_REWARD_ITEM_TYPE: i32 = 1;
pub const DAILY_STORY_REWARD_COUNT: i32 = 50;
