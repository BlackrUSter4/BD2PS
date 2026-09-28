use bd2::prost::Message;
use bd2::proto::proto_net::{FishingMapBuyRequest, FishingMapBuyResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// The request carries no payment field at all (no `use_item_info`, unlike the shop
/// requests), so map unlocks are granted unconditionally here — whatever currency/quest
/// gate the real game applies must happen client-side or via a different flow.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingMapBuyRequest) -> GameResponse {
    info!("Handling FishingMapBuyRequest: {:?}", req);

    if let Some(map_id) = req.map_id {
        let _ = database::db::fishing::fishing_map_owned::add(pool, uid, map_id).await;
    }

    let response = FishingMapBuyResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingMapBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
