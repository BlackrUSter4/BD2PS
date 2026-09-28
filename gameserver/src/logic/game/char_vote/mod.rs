pub mod char_vote_favorite_add;
pub mod char_vote_favorite_delete;
pub mod char_vote_info;
pub mod char_vote_ranking;
pub mod char_vote_save;
pub mod char_vote_season_ranking;
pub mod char_vote_total_ranking;

use bd2::proto::proto_net::Notify;

/// No CharVoteRoundScheduleTable/event-schedule master data was captured (or exists in this
/// client's tables at all, as far as decompiling turned up) — there's exactly one always-active
/// placeholder event/round rather than a real rotating schedule.
pub const CURRENT_EVENT_ID: i32 = 1;
pub const CURRENT_ROUND: i32 = 1;

/// Milestone thresholds for `CharVoteCountRewardDBInfo` (cumulative votes -> reward_id), and the
/// placeholder grant per milestone — no CharVoteRewardTable exists anywhere in the captured/
/// decompiled data, so both the thresholds and the grant amount are documented placeholders.
/// Claim-tracking itself (never double-granting) is real.
pub const REWARD_MILESTONES: &[i32] = &[10, 50, 100, 250, 500, 1000];
pub const PLACEHOLDER_REWARD_ITEM_ID: i32 = 4; // gold, confirmed real from CurrencyTable
pub const PLACEHOLDER_REWARD_ITEM_TYPE: i32 = 1;
pub const PLACEHOLDER_REWARD_COUNT: i32 = 100;

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

/// The candidate roster: no CharVoteTable exists to say which characters are up for vote this
/// round, so the first 10 real character ids (by id order) from CharTable stand in — real ids,
/// placeholder selection.
pub fn candidate_ids() -> Vec<i32> {
    data::exceldb::get()
        .chartable
        .all()
        .iter()
        .map(|c| c.id)
        .take(10)
        .collect()
}

pub fn today_string() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

pub fn next_midnight_utc_millis() -> i64 {
    let now = chrono::Utc::now();
    let tomorrow = (now + chrono::Duration::days(1)).date_naive();
    tomorrow
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis()
}
