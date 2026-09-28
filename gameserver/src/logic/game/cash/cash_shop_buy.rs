use bd2::prost::Message;
use bd2::proto::proto_net::{CashShopBuyRequest, CashShopBuyResponse, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::purchase::purchase_count_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real purchase-count tracking (no real-money processing happens on a private server, and no
/// captured cash-shop master data links a product id to any reward contents — `cash_shop_info`
/// starter JSON only has scheduling fields, no reward catalog — so the grant itself is
/// honestly empty rather than fabricated).
pub async fn handle(pool: &SqlitePool, uid: i64, req: CashShopBuyRequest) -> GameResponse {
    info!("Handling CashShopBuyRequest: {:?}", req);

    if let (Some(group_id), Some(product_id)) = (req.group_id, req.product_id) {
        let sale_group = req.sale_group.unwrap_or(0);
        let _ = db::increment(pool, uid, group_id, product_id, sale_group).await;
    }

    let response = CashShopBuyResponse {
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
        first_auto_revive_set_char_inven_index: None,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
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
    };

    let (route, code) = PacketCodeType::CashShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
