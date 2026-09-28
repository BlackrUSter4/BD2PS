use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameDefenseEndRequest, MiniGameDefenseEndResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No real-time multiplayer session state exists to resolve a real result from (see
/// MiniGameRelayServerInfo) — reward stays honestly empty rather than fabricated.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: MiniGameDefenseEndRequest) -> GameResponse {
    info!("Handling MiniGameDefenseEndRequest: {:?}", req);

    let response = MiniGameDefenseEndResponse { reward_info_bundle: None, reward_info: vec![] };
    
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
    
    let (route, code) = PacketCodeType::MiniGameDefenseEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}