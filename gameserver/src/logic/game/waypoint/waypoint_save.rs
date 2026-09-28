use bd2::proto::proto_net::{Notify, WaypointSaveRequest, WaypointSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use prost::Message;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: WaypointSaveRequest) -> GameResponse {
    info!("Handling WaypointSaveRequest: {:?}", req);

    let waypoint_id = req.waypoint_id.unwrap_or_default();

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO WaypointInfo (Uid, WaypointId)
        VALUES (?, ?)
        ON CONFLICT(Uid) DO UPDATE SET WaypointId = excluded.WaypointId
        "#,
    )
    .bind(uid)
    .bind(waypoint_id)
    .execute(pool)
    .await
    {
        eprintln!("Failed to insert WaypointInfo for uid {}: {:?}", uid, e);
    }

    let response = WaypointSaveResponse {
    
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

    let (route, code) = PacketCodeType::WaypointSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
