use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperReconnectRequest, LifeHelperReconnectResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Exact intended semantics unclear from the schema alone (empty response, no fields to
/// reflect back) — likely re-establishing a helper's work assignment after some client-side
/// desync. Implemented as a no-op ack; revisit if real client behavior reveals more.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: LifeHelperReconnectRequest) -> GameResponse {
    info!("Handling LifeHelperReconnectRequest: {:?}", req);

    let response = LifeHelperReconnectResponse {};
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeHelperReconnect.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
