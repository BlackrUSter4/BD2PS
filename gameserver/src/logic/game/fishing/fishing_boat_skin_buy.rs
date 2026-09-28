use bd2::prost::Message;
use bd2::proto::proto_net::{FishingBoatSkinBuyRequest, FishingBoatSkinBuyResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Same as FishingMapBuy — no payment field in the request, so granted unconditionally.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingBoatSkinBuyRequest) -> GameResponse {
    info!("Handling FishingBoatSkinBuyRequest: {:?}", req);

    if let Some(skin_id) = req.skin_id {
        let _ = database::db::fishing::fishing_boat_skin_owned::add(pool, uid, skin_id).await;
    }

    let response = FishingBoatSkinBuyResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingBoatSkinBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
