use bd2::prost::Message;
use bd2::proto::proto_net::{ActiveMapRequest, ActiveMapResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::map::map_active_info::add_map_active_info;
use database::models::game::map::map_active_info::MapActiveInfo;
use serde_json::to_string;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ActiveMapRequest) -> GameResponse {
    info!("Handling ActiveMapRequest: {:?}", req);

    let active_info_json = to_string(&req.active_info).unwrap_or_else(|_| "[]".to_string());

    if let Some(id) = req.map_id {
        let record = MapActiveInfo {
            index: 0, // autoincrement
            uid,
            map_id: Some(id),
            active_info: active_info_json,
        };

        if let Err(e) = add_map_active_info(pool, &record).await {
            eprintln!(
                "Failed to insert active_info {} for uid {}: {:?}",
                id, uid, e
            );
        }
    }

    let response = ActiveMapResponse {
        // TODO: Fill in response fields
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

    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
