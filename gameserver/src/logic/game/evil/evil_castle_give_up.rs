use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleGiveUpRequest, EvilCastleGiveUpResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// The tower mode (unlike the roguelike run, which has its own
/// `EvilCastleRogueLikeGiveUp`) has no "attempt in progress" state anywhere in this
/// project's schema to clear — `StageIndex`/`Retry` on `EvilCastleInfo` are persistent
/// progression, not a session marker. A real no-op ack, not a fabricated state reset.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: EvilCastleGiveUpRequest) -> GameResponse {
    info!("Handling EvilCastleGiveUpRequest: {:?}", req);

    let response = EvilCastleGiveUpResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleGiveUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
