use bd2::prost::Message;
use bd2::proto::proto_net::{LeaveUserRequest, LeaveUserResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::user::user_info as user_db;
use sqlx::SqlitePool;
use tracing::info;

const LEAVE_GRACE_PERIOD_MS: i64 = 7 * 24 * 60 * 60 * 1000;

/// Real account-deletion request: sets the real UnregDate a fixed grace period out (the
/// counterpart to CancelLeaveUser, which clears it).
pub async fn handle(pool: &SqlitePool, uid: i64, req: LeaveUserRequest) -> GameResponse {
    info!("Handling LeaveUserRequest: {:?}", req);

    let unreg_date = chrono::Utc::now().timestamp_millis() + LEAVE_GRACE_PERIOD_MS;
    let _ = user_db::set_unreg_date(pool, uid, unreg_date).await;

    let response = LeaveUserResponse { unreg_date: Some(unreg_date) };
    
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
    
    let (route, code) = PacketCodeType::LeaveUser.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}