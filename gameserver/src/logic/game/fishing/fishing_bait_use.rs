use bd2::prost::Message;
use bd2::proto::proto_net::{FishingBaitUseRequest, FishingBaitUseResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Consumes from the shared/generic item inventory (the request carries a plain
/// `proto.net.ItemDBInfo`, not a Fishing-specific one) — bait is evidently just a regular
/// item, not tracked in FishingItemInfo.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingBaitUseRequest) -> GameResponse {
    info!("Handling FishingBaitUseRequest: {:?}", req);

    if let Some(item) = &req.use_item_info {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = database::db::item::item_info::consume(pool, uid, id, count.max(1)).await;
        }
    }

    let response = FishingBaitUseResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingBaitUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
