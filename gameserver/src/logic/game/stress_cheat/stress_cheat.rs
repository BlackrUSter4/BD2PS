use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, StressCheatRequest, StressCheatResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// A debug/stress-testing endpoint (empty request/response, "Cheat" suffix matches this
/// project's other debug-only routes like LifeCheat) — no state anywhere models what it
/// would even mean to persist, so this is a genuine no-op ack, not a fabricated effect.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: StressCheatRequest) -> GameResponse {
    info!("Handling StressCheatRequest: {:?}", req);

    let response = StressCheatResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::StressCheat.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
