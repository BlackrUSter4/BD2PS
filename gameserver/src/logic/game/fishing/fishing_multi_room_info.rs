use bd2::prost::Message;
use bd2::proto::proto_net::{FishingMultiRoomInfoRequest, FishingMultiRoomInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No real multiplayer/matchmaking backend exists in this project — an empty room list is
/// the honest answer here (there is genuinely nothing to list), not a placeholder.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: FishingMultiRoomInfoRequest) -> GameResponse {
    info!("Handling FishingMultiRoomInfoRequest: {:?}", req);

    let response = FishingMultiRoomInfoResponse { room_info: vec![] };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingMultiRoomInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
