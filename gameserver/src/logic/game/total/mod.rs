use bd2::proto::proto_net::Notify;

pub const GOLD_ITEM_ID: i32 = 4;
pub const GOLD_ITEM_TYPE: i32 = 1;

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

use bd2::proto::proto_net::BattleDamageDbInfo;
use database::models::game::total::total_war_info::TotalWarInfo;

/// Parse the account's stored per-category score list (JSON, one BattleDamageDbInfo per
/// category id — matches the proto shape 1:1 rather than collapsing to a single total,
/// since the client can submit multiple score categories per battle).
pub fn parse_scores(row: &TotalWarInfo) -> Vec<BattleDamageDbInfo> {
    row.score_info_index
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default()
}

/// Merge newly-submitted score deltas into the account's stored per-category totals
/// (additive — each submission adds to that category's running total).
pub fn merge_scores(existing: &mut Vec<BattleDamageDbInfo>, incoming: &[BattleDamageDbInfo]) {
    for delta in incoming {
        let id = delta.id.unwrap_or(0);
        let value = delta.value.unwrap_or(0);
        if let Some(entry) = existing.iter_mut().find(|e| e.id == Some(id)) {
            entry.value = Some(entry.value.unwrap_or(0) + value);
        } else {
            existing.push(BattleDamageDbInfo {
                id: Some(id),
                value: Some(value),
            });
        }
    }
}

pub fn total_score(scores: &[BattleDamageDbInfo]) -> i64 {
    scores.iter().map(|s| s.value.unwrap_or(0)).sum()
}

/// Real cross-account top score + this account's percentile (never bot-padded, same
/// pattern as every other ranked system in this project).
pub async fn compute_rank(pool: &sqlx::SqlitePool, uid: i64) -> (i64, f64) {
    let all = database::db::total::total_war_info::all_rows(pool)
        .await
        .unwrap_or_default();
    let mut scored: Vec<(i64, i64)> = all
        .iter()
        .map(|r| (r.uid, total_score(&parse_scores(r))))
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1));
    let top = scored.first().map(|(_, s)| *s).unwrap_or(0);
    let total = scored.len().max(1);
    let pos = scored.iter().position(|(u, _)| *u == uid).unwrap_or(total - 1);
    (top, ((pos + 1) as f64 / total as f64) * 100.0)
}

pub mod total_ranking;
pub mod total_war_battle_end;
pub mod total_war_battle_start;
pub mod total_war_contents_item_renew;
pub mod total_war_deck_info;
pub mod total_war_deck_preset_info;
pub mod total_war_deck_preset_save;
pub mod total_war_deck_preset_slot_add;
pub mod total_war_deck_save;
pub mod total_war_info;
pub mod total_war_preset_delete;
pub mod total_war_preset_info_change;
pub mod total_war_reward;
pub mod total_war_reward_state;
