use bd2::prost::Message;
use bd2::proto::proto_net::{LifeCheatRegenRequest, LifeCheatRegenResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Debug/cheat endpoint (empty request and response by schema) — acknowledged as a no-op.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: LifeCheatRegenRequest) -> GameResponse {
    info!("Handling LifeCheatRegenRequest: {:?}", req);

    let response = LifeCheatRegenResponse {};
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeCheatRegen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
