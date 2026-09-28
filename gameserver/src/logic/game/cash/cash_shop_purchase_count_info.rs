use bd2::prost::Message;
use bd2::proto::proto_net::{CashShopPurchaseCountInfoRequest, CashShopPurchaseCountInfoResponse, Notify, PurchaseCountDbInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::purchase::purchase_count_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account, per-product purchase counts (already had full db scaffolding — just
/// needed gameserver glue).
pub async fn handle(pool: &SqlitePool, uid: i64, req: CashShopPurchaseCountInfoRequest) -> GameResponse {
    info!("Handling CashShopPurchaseCountInfoRequest: {:?}", req);

    let purchase_count_info = db::get_purchase_count_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| PurchaseCountDbInfo { group_id: r.group_id, id: r.id, sale_group: r.sale_group, count: r.count })
        .collect();

    let response = CashShopPurchaseCountInfoResponse { purchase_count_info };

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

    let (route, code) = PacketCodeType::CashShopPurchaseCountInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
