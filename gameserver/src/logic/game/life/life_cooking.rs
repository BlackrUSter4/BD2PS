use super::try_consume_items;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeCookingRequest, LifeCookingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Consumes the claimed ingredients for real. The actual output item is governed by
/// `LifeCookTable`, which has no captured master data yet (see CLIENT_UPDATE.md) — so the
/// reward bundle is left empty rather than fabricated. Ingredients are still spent, since not
/// doing so would be worse (free re-tries) than a missing reward.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeCookingRequest) -> GameResponse {
    info!("Handling LifeCookingRequest: {:?}", req);

    let _consumed = try_consume_items(pool, uid, &req.use_item_info).await;

    let response = LifeCookingResponse {
        reward_info_bundle: None,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeCooking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
