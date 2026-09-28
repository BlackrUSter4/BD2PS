use bd2::prost::Message;
use bd2::proto::proto_net::{ClientCustomLogRequest, ClientCustomLogResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Client telemetry ack — nothing to persist for a private server (no analytics pipeline
/// exists or is wanted here); an empty response is the whole real behavior.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: ClientCustomLogRequest) -> GameResponse {
    info!("Handling ClientCustomLogRequest: {} log entries", req.log_data.len());

    let response = ClientCustomLogResponse {};
    
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
    
    let (route, code) = PacketCodeType::ClientCustomLog.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}