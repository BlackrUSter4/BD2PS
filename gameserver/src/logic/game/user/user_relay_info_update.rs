use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, UserRelayInfoUpdateRequest, UserRelayInfoUpdateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Empty request and response (only `seq`) — a real no-op ack, correct-as-empty by the
/// proto's own design, same category as PingCheck/LogoutUser.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: UserRelayInfoUpdateRequest) -> GameResponse {
    info!("Handling UserRelayInfoUpdateRequest: {:?}", req);

    let response = UserRelayInfoUpdateResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::UserRelayInfoUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
