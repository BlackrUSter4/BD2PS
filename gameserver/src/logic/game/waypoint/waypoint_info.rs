use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, WaypointInfoRequest, WaypointInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::waypoint::waypoint_info::get_waypoint_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, _req: WaypointInfoRequest) -> GameResponse {
    info!("Handling WaypointInfoRequest for uid: {}", uid);

    let waypoint_rows = match get_waypoint_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Error fetching WaypointInfo: {:?}", err);
            vec![]
        }
    };

    let waypoint_ids: Vec<i32> = waypoint_rows
        .into_iter()
        .map(|row| row.waypoint_id)
        .collect();

    let response = WaypointInfoResponse {
        waypoint_id: waypoint_ids,
        ..Default::default()
    };

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

    let (route, code) = PacketCodeType::WaypointInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
