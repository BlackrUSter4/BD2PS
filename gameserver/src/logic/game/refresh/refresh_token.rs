use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, RefreshTokenRequest, RefreshTokenResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Real new-token issuance, same generation approach as the REST LoginUser route (random
/// 64-char token) — this server has no expiring-session concept to actually validate against,
/// so any refresh request is honored.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: RefreshTokenRequest) -> GameResponse {
    info!("Handling RefreshTokenRequest: {:?}", req);

    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let access_token: String = (0..64).map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char).collect();

    let response = RefreshTokenResponse { access_token: Some(access_token) };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::RefreshToken.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
