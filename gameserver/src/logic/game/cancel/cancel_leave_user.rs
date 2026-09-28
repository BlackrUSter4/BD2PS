use bd2::prost::Message;
use bd2::proto::proto_net::{CancelLeaveUserRequest, CancelLeaveUserResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::user::user_info as user_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real cancellation: clears the account's real pending-deletion timestamp.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CancelLeaveUserRequest) -> GameResponse {
    info!("Handling CancelLeaveUserRequest: {:?}", req);

    let _ = user_db::clear_unreg_date(pool, uid).await;

    let response = CancelLeaveUserResponse {};
    
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
    
    let (route, code) = PacketCodeType::CancelLeaveUser.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}