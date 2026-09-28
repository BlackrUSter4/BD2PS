use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PackEventStoryDbInfo, PackEventStoryInfoRequest, PackEventStoryInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pack::pack_event_story_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account cleared-story progress, filtered by the requested event_uid list.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PackEventStoryInfoRequest) -> GameResponse {
    info!("Handling PackEventStoryInfoRequest: {:?}", req);

    let rows = db::get_pack_event_story_info(pool, uid).await.unwrap_or_default();
    let info = rows
        .into_iter()
        .filter(|r| req.event_uid.is_empty() || r.event_uid.map(|e| req.event_uid.contains(&e)).unwrap_or(false))
        .map(|r| PackEventStoryDbInfo {
            event_uid: r.event_uid,
            group_id: r.group_id,
            id: r.id,
        })
        .collect();

    let response = PackEventStoryInfoResponse { info };

    let resp_bytes = response.encode_to_vec();

    // Notify same as others
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::PackEventStoryInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
