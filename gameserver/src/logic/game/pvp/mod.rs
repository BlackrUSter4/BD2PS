pub mod pvp_battle_deck_info;
pub mod pvp_battle_deck_save;
pub mod pvp_battle_end;
pub mod pvp_battle_history;
pub mod pvp_battle_history_deck_info;
pub mod pvp_battle_matching;
pub mod pvp_battle_once_reward_info;
pub mod pvp_battle_ranking;
pub mod pvp_battle_rank_user_detail;
pub mod pvp_battle_replay_info;
pub mod pvp_battle_reset;
pub mod pvp_battle_reward;
pub mod pvp_battle_start;
pub mod pvp_battle_user_info;
pub mod pvp_contents_item_renew;
pub mod pvp_season_reward;

use bd2::proto::proto_net::{ContentsCharItemInfo, ContentsEquipDbInfo, ItemDbInfo, Notify};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub const BATTLE_RESULT_WIN: i32 = 1;

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

/// Same established convention as SpineInteraction/CharVote/Friendship: no table anywhere maps
/// a PvP reward `type`/`id` pair to a real item, so the count-bearing reward arrays in
/// `PvpRankTable`/`PvpDefaultTable` (which ARE real, captured data) are granted as real gold
/// (item id 4 / type 1, confirmed via `CurrencyTable`) using the table's real count value.
pub const GOLD_ITEM_ID: i32 = 4;
pub const GOLD_ITEM_TYPE: i32 = 1;

/// Current PvP season, derived from the real captured `PvpSeasonTable` (3 rows) rather than a
/// placeholder constant — the latest defined season is treated as "current".
pub fn current_season() -> i32 {
    data::exceldb::get()
        .pvpseasontable
        .all()
        .iter()
        .map(|s| s.season)
        .max()
        .unwrap_or(1)
}

/// The `PvpRankTable` row whose `vp` threshold is the highest one at or below the given Vp
/// (falling back to the lowest-threshold row, then any row, if none qualify) — real per-bracket
/// win/lose points and reward data, not a placeholder formula.
pub fn rank_row_for_vp(vp: i32) -> Option<&'static data::exceldb::pvpranktable::Pvpranktable> {
    let table = &data::exceldb::get().pvpranktable;
    table
        .all()
        .iter()
        .filter(|r| r.vp <= vp)
        .max_by_key(|r| r.vp)
        .or_else(|| table.all().iter().min_by_key(|r| r.vp))
}

/// Real Vp gained on a win, from the caller's rank bracket.
pub fn win_point_for_vp(vp: i32) -> i32 {
    rank_row_for_vp(vp).map(|r| r.win_point).unwrap_or(20)
}

/// Real Vp *lost* on a loss, already negated for direct use as a delta
/// (`PvpRankTable.losePoint` is stored as a positive magnitude).
pub fn lose_point_for_vp(vp: i32) -> i32 {
    -rank_row_for_vp(vp).and_then(|r| r.lose_point).unwrap_or(15)
}

fn gold_item(count: i32) -> ItemDbInfo {
    ItemDbInfo {
        inven_index: None,
        id: Some(GOLD_ITEM_ID),
        r#type: Some(GOLD_ITEM_TYPE),
        count: Some(count),
        keep_flag: None,
        time_value: None,
        pictorialbook_info: None,
        expiry_time: None,
        sort_id: None,
        use_count: None,
    }
}

/// Real reward count from the caller's rank bracket (`PvpRankTable`'s win/lose reward count
/// arrays), granted as gold per the module-level convention. Also performs the actual grant.
pub async fn grant_battle_reward(pool: &SqlitePool, uid: i64, vp: i32, won: bool) -> Vec<ItemDbInfo> {
    let Some(row) = rank_row_for_vp(vp) else { return vec![] };
    let counts = if won { &row.battle_win_reward_count } else { &row.battle_lose_reward_count };
    let total: i32 = counts.iter().sum();
    if total <= 0 {
        return vec![];
    }
    let _ = database::db::item::item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, total).await;
    vec![gold_item(total)]
}

/// Real reward count from the caller's rank bracket's season-reward array.
pub async fn grant_season_reward(pool: &SqlitePool, uid: i64, vp: i32) -> Vec<ItemDbInfo> {
    let Some(row) = rank_row_for_vp(vp) else { return vec![] };
    let total: i32 = row.season_reward_count.iter().sum();
    if total <= 0 {
        return vec![];
    }
    let _ = database::db::item::item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, total).await;
    vec![gold_item(total)]
}

/// Once-per-account VP-threshold rewards, real thresholds sourced from every distinct `vp`
/// value in `PvpRankTable` (81 rows) rather than an invented tier list. Grants any newly-crossed
/// tier (by comparing old/new Vp) and returns the full list of tier ids ever claimed.
pub async fn grant_new_once_rewards(pool: &SqlitePool, uid: i64, old_vp: i32, new_vp: i32) -> Vec<i32> {
    let table = &data::exceldb::get().pvpranktable;
    let mut thresholds: Vec<i32> = table.all().iter().map(|r| r.vp).collect();
    thresholds.sort_unstable();
    thresholds.dedup();

    let user = database::db::pvp::pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let mut claimed: Vec<i32> = parse_claimed(&user.once_reward_claimed);

    for (tier_id, threshold) in thresholds.iter().enumerate() {
        let tier_id = tier_id as i32;
        if new_vp >= *threshold && old_vp < *threshold && !claimed.contains(&tier_id) {
            claimed.push(tier_id);
            if let Some(row) = rank_row_for_vp(*threshold) {
                let count: i32 = row.battle_win_reward_count.iter().sum();
                if count > 0 {
                    let _ = database::db::item::item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, count).await;
                }
            }
        }
    }

    let csv = claimed.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",");
    let _ = database::db::pvp::pvp_user_info::set_once_reward_claimed(pool, uid, &csv).await;
    claimed
}

pub fn parse_claimed(csv: &str) -> Vec<i32> {
    if csv.is_empty() {
        vec![]
    } else {
        csv.split(',').filter_map(|s| s.parse().ok()).collect()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct StoredCharItem {
    pub char_inven_index: i64,
    pub connect_potential_costume: Option<i32>,
    pub equips: Vec<(i32, i64)>, // (equip_type, equip_inven_index)
}

pub fn items_to_json(items: &[ContentsCharItemInfo]) -> String {
    let stored: Vec<StoredCharItem> = items
        .iter()
        .map(|item| StoredCharItem {
            char_inven_index: item.char_inven_index.unwrap_or(0),
            connect_potential_costume: item.connect_potential_costume,
            equips: item
                .equip_info
                .iter()
                .map(|e| (e.equip_type.unwrap_or(0), e.equip_inven_index.unwrap_or(0)))
                .collect(),
        })
        .collect();
    serde_json::to_string(&stored).unwrap_or_else(|_| "[]".to_string())
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct DeckSlotSnapshot {
    pub char_inven_index: i64,
    pub position: i32,
    pub sequence: Option<i32>,
    pub costume_inven_index: Option<i64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DeckSnapshot {
    pub my: Vec<DeckSlotSnapshot>,
    pub enemy: Vec<DeckSlotSnapshot>,
}

/// Builds a replay-able snapshot: the caller's real attack deck, and either the real opponent's
/// real attack deck or synthetic slots built from the bot's real `CharTable` ids.
pub async fn build_deck_snapshot(pool: &SqlitePool, uid: i64, match_info: &Option<database::models::game::pvp::pvp_current_match::PvpCurrentMatch>) -> String {
    let my = database::db::pvp::pvp_deck_info::get_by_uid_type(pool, uid, 0)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| DeckSlotSnapshot {
            char_inven_index: d.char_inven_index,
            position: d.position,
            sequence: d.sequence,
            costume_inven_index: d.costume_inven_index,
        })
        .collect();

    let enemy = match match_info {
        Some(m) if !m.enemy_is_bot => {
            if let Some(enemy_uid) = m.enemy_owner_index {
                database::db::pvp::pvp_deck_info::get_by_uid_type(pool, enemy_uid, 1)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|d| DeckSlotSnapshot {
                        char_inven_index: d.char_inven_index,
                        position: d.position,
                        sequence: d.sequence,
                        costume_inven_index: d.costume_inven_index,
                    })
                    .collect()
            } else {
                vec![]
            }
        }
        Some(m) => m
            .enemy_char_ids
            .as_deref()
            .unwrap_or("")
            .split(',')
            .filter(|s| !s.is_empty())
            .enumerate()
            .filter_map(|(pos, id)| {
                id.parse::<i64>().ok().map(|char_id| DeckSlotSnapshot {
                    char_inven_index: -(char_id + 1),
                    position: pos as i32,
                    sequence: Some(0),
                    costume_inven_index: None,
                })
            })
            .collect(),
        None => vec![],
    };

    serde_json::to_string(&DeckSnapshot { my, enemy }).unwrap_or_else(|_| "{}".to_string())
}

pub fn json_to_items(json: &str) -> Vec<ContentsCharItemInfo> {
    let stored: Vec<StoredCharItem> = serde_json::from_str(json).unwrap_or_default();
    stored
        .into_iter()
        .map(|s| ContentsCharItemInfo {
            char_inven_index: Some(s.char_inven_index),
            equip_info: s
                .equips
                .into_iter()
                .map(|(t, i)| ContentsEquipDbInfo { equip_inven_index: Some(i), equip_type: Some(t) })
                .collect(),
            connect_potential_costume: s.connect_potential_costume,
        })
        .collect()
}
