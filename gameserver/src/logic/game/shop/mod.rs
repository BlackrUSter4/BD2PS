pub mod shop_buy;
pub mod shop_info;
pub mod shop_open;
pub mod shop_sell;

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

/// Real per-shop reset duration from ShopTable (resetCount interpreted as a day count — the
/// only unit consistent with resetTermType's real values captured so far).
pub fn reset_seconds_for(shop: &data::exceldb::shoptable::Shoptable) -> i32 {
    (shop.reset_count.max(1)) * 86_400
}
