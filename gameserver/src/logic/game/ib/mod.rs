pub mod ib_deck_info;
pub mod ib_deck_save;
pub mod ib_dungeon_enter;
pub mod ib_dungeon_give_up;
pub mod ib_item_info;
pub mod ib_item_sell;
pub mod ib_item_upgrade;
pub mod ib_main_info;
pub mod ib_shop_buy;
pub mod ib_shop_info;
pub mod ib_shop_item_reserve;
pub mod ib_shop_refresh;
pub mod ib_stage_end;
pub mod ib_stage_start;

use bd2::proto::proto_net::{IbDeckDbInfo, IbItemDbInfo, IbPlayDbInfo, IbShopItemDbInfo, Notify};
use database::db::ib::ib_play_state;
use database::models::game::ib::ib_play_state::IbPlayState;
use sqlx::SqlitePool;

/// No `IBDefaultTable`/`IBDungeonTable`/`IBSeasonTable`/`IBStageTable`/etc. data was captured
/// from the live client (schemas confirmed via decompiled client, zero rows — the account used
/// for capture never opened this feature). Every constant below is a documented, reasonable
/// placeholder standing in for what those tables would otherwise supply. Revisit if that data
/// ever gets captured.
pub const CURRENT_SEASON: i32 = 1;
pub const REGULAR_SEASON: i32 = 1;
pub const STARTING_LIFE: i32 = 3;
pub const SHOP_SLOT_COUNT: i32 = 6;
pub const ITEM_SELL_COIN_PER_LEVEL: i32 = 10;
pub const ITEM_UPGRADE_COIN_PER_LEVEL: i32 = 20;
pub const SHOP_REFRESH_COST: i32 = 30;
pub const STAGE_CLEAR_COIN: i32 = 50;
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

pub fn play_info_proto(state: &IbPlayState) -> IbPlayDbInfo {
    IbPlayDbInfo {
        dungeon_id: Some(state.dungeon_id),
        stage_id: Some(state.stage_id),
        life: Some(state.life),
        coin: Some(state.coin),
        season: Some(state.season),
        shop_reload_count: Some(state.shop_reload_count),
        is_stage_enter: Some(state.is_stage_enter),
    }
}

pub fn deck_info_proto(rows: &[database::models::game::ib::ib_deck::IbDeck]) -> Vec<IbDeckDbInfo> {
    rows.iter()
        .map(|d| IbDeckDbInfo {
            inven_index: Some(d.inven_index),
            position: Some(d.position),
            rotation_count: Some(d.rotation_count),
        })
        .collect()
}

pub fn item_info_proto(rows: &[database::models::game::ib::ib_inventory::IbInventory]) -> Vec<IbItemDbInfo> {
    rows.iter()
        .map(|i| IbItemDbInfo {
            inven_index: Some(i.inven_index),
            r#type: Some(i.r#type),
            id: Some(i.item_id),
            level: Some(i.level),
        })
        .collect()
}

pub fn shop_item_info_proto(rows: &[database::models::game::ib::ib_shop::IbShop]) -> Vec<IbShopItemDbInfo> {
    rows.iter()
        .map(|s| IbShopItemDbInfo {
            slot: Some(s.slot),
            r#type: Some(s.r#type),
            id: Some(s.item_id),
            price: Some(s.price),
            original_price: Some(s.original_price),
            is_discount: Some(s.is_discount),
            is_reserved: Some(s.is_reserved),
            is_sold_out: Some(s.is_sold_out),
        })
        .collect()
}

/// Deterministic-ish placeholder shop roll (no `IBItemTable`/`IBShopTable` data exists to draw
/// real items/prices from). Slot `n` gets item id `n + 1`, a price scaling with slot, and every
/// 3rd slot discounted — purely to make the shop screen functional, not a real drop table.
pub fn generate_shop_rows(uid: i64) -> Vec<database::models::game::ib::ib_shop::IbShop> {
    (0..SHOP_SLOT_COUNT)
        .map(|slot| {
            let original_price = 100 + slot * 20;
            let is_discount = slot % 3 == 2;
            let price = if is_discount { original_price - original_price / 5 } else { original_price };
            database::models::game::ib::ib_shop::IbShop {
                uid,
                slot,
                r#type: 0,
                item_id: slot + 1,
                price,
                original_price,
                is_discount,
                is_reserved: false,
                is_sold_out: false,
            }
        })
        .collect()
}

/// Ensures a season change resets dungeon-run progress. Returns whether a reset just happened
/// (for `IbMainInfoResponse.is_season_reset`), and the (possibly-updated) play state.
pub async fn ensure_season(pool: &SqlitePool, uid: i64) -> (bool, IbPlayState) {
    let state = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();
    if state.season != CURRENT_SEASON {
        let _ = ib_play_state::reset_for_new_season(pool, uid, CURRENT_SEASON, STARTING_LIFE).await;
        let refreshed = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();
        (true, refreshed)
    } else {
        (false, state)
    }
}
