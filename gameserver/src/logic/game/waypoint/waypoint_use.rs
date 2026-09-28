use bd2::prost::Message;
use bd2::proto::proto_net::{WaypointUseRequest, WaypointUseResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::waypoint::waypoint_info as db;
use database::models::game::waypoint::waypoint_info::WaypointInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real fast-travel unlock: marks the destination waypoint visited (idempotent) using the
/// already-scaffolded WaypointInfo table. No move-point/stamina resource column exists
/// anywhere in this project to deduct `move_point` from, so it's accepted but not charged.
pub async fn handle(pool: &SqlitePool, uid: i64, req: WaypointUseRequest) -> GameResponse {
    info!("Handling WaypointUseRequest: {:?}", req);

    if let Some(end_waypoint_id) = req.end_waypoint_id {
        let already = db::get_waypoint_info(pool, uid).await.unwrap_or_default().into_iter().any(|r| r.waypoint_id == end_waypoint_id);
        if !already {
            let _ = db::add_waypoint_info(pool, &WaypointInfo { index: 0, uid, waypoint_id: end_waypoint_id }).await;
        }
    }

    let response = WaypointUseResponse {};
    
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
    
    let (route, code) = PacketCodeType::WaypointUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}