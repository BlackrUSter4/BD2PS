use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperGachaRequest, LifeHelperGachaResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// The helper candidate pool/weights live in `LifeHelperSpeciesTable`, which has no captured
/// master data yet — returning a genuinely empty roll (and not charging anything for it) is
/// more honest than fabricating candidate helpers.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: LifeHelperGachaRequest) -> GameResponse {
    info!("Handling LifeHelperGachaRequest: {:?}", req);

    let response = LifeHelperGachaResponse {
        helper_info: vec![],
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

    let (route, code) = PacketCodeType::LifeHelperGacha.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
