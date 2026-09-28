pub mod colosseum_ap_buy;
pub mod colosseum_battle_count_reset;
pub mod colosseum_battle_end;
pub mod colosseum_battle_history;
pub mod colosseum_battle_matching;
pub mod colosseum_battle_replay_info;
pub mod colosseum_battle_start;
pub mod colosseum_bless_info;
pub mod colosseum_bless_save;
pub mod colosseum_contents_item_renew;
pub mod colosseum_deck_info;
pub mod colosseum_deck_save;
pub mod colosseum_preset_delete;
pub mod colosseum_preset_info;
pub mod colosseum_preset_info_change;
pub mod colosseum_preset_save;
pub mod colosseum_preset_slot_add;
pub mod colosseum_preset_use;
pub mod colosseum_promotion_reward_info;
pub mod colosseum_rank_detail;
pub mod colosseum_ranking;
pub mod colosseum_season_reward;
pub mod colosseum_user_info;

use bd2::proto::proto_net::{
    ColosseumDeckInfo as ColosseumDeckInfoProto, ColosseumUserBaseInfo, ContentsCharItemInfo,
    ContentsEquipDbInfo, Notify, RewardDbInfoBundle,
};
use database::db::colosseum::colosseum_deck_char_item_info as deck_item_db;
use database::db::colosseum::colosseum_deck_info as deck_db;
use database::models::game::colosseum::colosseum_deck_char_equip_info::ColosseumDeckCharEquipInfo;
use database::models::game::colosseum::colosseum_deck_char_item_info::ColosseumDeckCharItemInfo;
use database::models::game::colosseum::colosseum_user_info::ColosseumUserInfo as ColosseumUserInfoRow;
use sqlx::SqlitePool;

pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub const DAY_MS: i64 = 86_400_000;

/// No `ColosseumDefaultTable` row was captured from the live client (the table's schema
/// exists — confirmed via the decompiled client — but the account used for capture never
/// opened Colosseum). Every constant below is a documented, reasonable placeholder standing
/// in for what that table would otherwise supply. Revisit if that data ever gets captured.
pub const CURRENT_SEASON: i32 = 1;
pub const REGULAR_SEASON: i32 = 1;
pub const RANK_TABLE_CHANGE_SEASON: i32 = 1;
pub const CHANGE_VP_WIN: i32 = 20;
pub const CHANGE_VP_LOSE: i32 = -15;
pub const AP_BUY_MAX_PER_DAY: i32 = 5;
pub const MATCH_CANDIDATE_COUNT: i32 = 5;
pub const BATTLE_HISTORY_LIMIT: i32 = 20;
pub const BATTLE_RESULT_WIN: i32 = 1;

/// (Vp threshold, reward id) — placeholder promotion tiers; no `ColosseumRankTable` data was
/// captured either, so tier thresholds/ids can't be sourced from real data yet.
pub const PROMOTION_TIERS: &[(i32, i32)] = &[(1000, 1), (1200, 2), (1500, 3), (2000, 4), (2500, 5)];

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

pub fn empty_reward_bundle() -> RewardDbInfoBundle {
    RewardDbInfoBundle::default()
}

pub fn user_base_info(user: &ColosseumUserInfoRow, rank: i32) -> ColosseumUserBaseInfo {
    let total = (user.win_count + user.lose_count).max(1);
    ColosseumUserBaseInfo {
        vp: Some(user.vp),
        rank: Some(rank),
        win_count: Some(user.win_count),
        lose_count: Some(user.lose_count),
        top_percent: Some(100.0 * rank as f64 / total.max(rank) as f64),
    }
}

pub async fn build_deck_info_list(pool: &SqlitePool, uid: i64) -> Vec<ColosseumDeckInfoProto> {
    deck_db::get_by_uid(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| ColosseumDeckInfoProto {
            char_inven_index: Some(d.char_inven_index),
            position: Some(d.position),
            sequence: d.sequence,
            costume_inven_index: d.costume_inven_index,
        })
        .collect()
}

pub async fn build_deck_item_info_list(pool: &SqlitePool, uid: i64) -> Vec<ContentsCharItemInfo> {
    let items = deck_item_db::get_items_by_uid(pool, uid).await.unwrap_or_default();
    let equips = deck_item_db::get_equips_by_uid(pool, uid).await.unwrap_or_default();
    items
        .into_iter()
        .map(|item: ColosseumDeckCharItemInfo| {
            let char_equips: Vec<ContentsEquipDbInfo> = equips
                .iter()
                .filter(|e: &&ColosseumDeckCharEquipInfo| e.char_inven_index == item.char_inven_index)
                .map(|e| ContentsEquipDbInfo {
                    equip_inven_index: e.equip_inven_index,
                    equip_type: e.equip_type,
                })
                .collect();
            ContentsCharItemInfo {
                char_inven_index: Some(item.char_inven_index),
                equip_info: char_equips,
                connect_potential_costume: item.connect_potential_costume,
            }
        })
        .collect()
}

/// Saves a full deck replacement: deck slots + per-char item/equip info, in one go.
pub async fn save_deck(
    pool: &SqlitePool,
    uid: i64,
    deck_info: &[ColosseumDeckInfoProto],
    items: &[ContentsCharItemInfo],
) -> sqlx::Result<()> {
    let slots: Vec<deck_db::DeckSlot> = deck_info
        .iter()
        .enumerate()
        .map(|(i, d)| deck_db::DeckSlot {
            position: d.position.unwrap_or(i as i32),
            char_inven_index: d.char_inven_index.unwrap_or(0),
            sequence: d.sequence,
            costume_inven_index: d.costume_inven_index,
        })
        .collect();
    deck_db::replace_all(pool, uid, &slots).await?;

    let item_slots: Vec<deck_item_db::CharItemSlot> = items
        .iter()
        .map(|item| deck_item_db::CharItemSlot {
            char_inven_index: item.char_inven_index.unwrap_or(0),
            connect_potential_costume: item.connect_potential_costume,
            equips: item
                .equip_info
                .iter()
                .map(|e| (e.equip_type, e.equip_inven_index))
                .collect(),
        })
        .collect();
    deck_item_db::replace_all(pool, uid, &item_slots).await?;
    Ok(())
}

/// Promotion-tier rewards newly crossed by a Vp change (granted once each, ever).
pub async fn grant_new_promotion_rewards(pool: &SqlitePool, uid: i64, new_vp: i32) {
    for (threshold, reward_id) in PROMOTION_TIERS {
        if new_vp >= *threshold {
            let _ = database::db::colosseum::colosseum_promotion_reward::grant(pool, uid, *reward_id)
                .await;
        }
    }
}
