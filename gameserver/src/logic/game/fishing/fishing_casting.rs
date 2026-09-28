use bd2::prost::Message;
use bd2::proto::proto_net::{FishingCastingRequest, FishingCastingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Nothing else references `casting_id` server-side, and the response has no fields — a
/// genuine no-op acknowledgement, not a placeholder gap.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: FishingCastingRequest) -> GameResponse {
    info!("Handling FishingCastingRequest: {:?}", req);

    let response = FishingCastingResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingCasting.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
