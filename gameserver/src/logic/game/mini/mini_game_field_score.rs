use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameFieldScoreRequest, MiniGameFieldScoreResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No per-session score/HP state is tracked server-side for this minigame (client-simulates/
/// server-trusts, same as every other minigame in this project) — honest zero rather than
/// fabricated.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: MiniGameFieldScoreRequest) -> GameResponse {
    info!("Handling MiniGameFieldScoreRequest: {:?}", req);

    let response = MiniGameFieldScoreResponse { current_score: Some(0), current_hp: Some(0) };
    
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
    
    let (route, code) = PacketCodeType::MiniGameFieldScore.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}