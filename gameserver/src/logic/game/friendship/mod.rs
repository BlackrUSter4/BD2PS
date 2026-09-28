pub mod friendship_counseling;
pub mod friendship_gift;
pub mod friendship_info;
pub mod friendship_special_episode_clear;
pub mod friendship_special_episode_info;

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

/// Real, from `FriendshipDefaultTable`'s single row: correct/incorrect counseling-answer exp,
/// and the daily-completion bonus (type/count). No table anywhere links reward TYPE 3 to a real
/// item id, so — same convention SpineInteraction's round established — item id 4 / type 1
/// (confirmed real gold, `CurrencyTable` id 4) is used as the actual granted item; only the
/// choice of *which item* represents "type 3" is a placeholder, the exp/count values themselves
/// are real captured data.
pub const COUNSELING_CORRECT_EXP: i32 = 100;
pub const COUNSELING_INCORRECT_EXP: i32 = 80;
pub const COUNSELING_COMPLETE_REWARD_ITEM_ID: i32 = 4;
pub const COUNSELING_COMPLETE_REWARD_ITEM_TYPE: i32 = 1;
pub const COUNSELING_COMPLETE_REWARD_COUNT: i32 = 100;

/// No gift-value table exists (nothing links a gifted item to an exp amount) — flat placeholder
/// exp per item consumed.
pub const GIFT_EXP_PER_ITEM: i32 = 20;

/// No special-episode reward table exists — placeholder gold grant on first clear, same
/// item id/type convention as the counseling completion bonus.
pub const SPECIAL_EPISODE_REWARD_ITEM_ID: i32 = 4;
pub const SPECIAL_EPISODE_REWARD_ITEM_TYPE: i32 = 1;
pub const SPECIAL_EPISODE_REWARD_COUNT: i32 = 50;
