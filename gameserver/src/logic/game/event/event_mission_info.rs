use bd2::prost::Message;
use bd2::proto::proto_net::{
    AchievementUpdateInfo, EventMissionDbInfo, EventMissionInfoRequest, EventMissionInfoResponse,
    Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::event::event_mission_info::get_event_mission_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EventMissionInfoRequest) -> GameResponse {
    info!("Handling EventMissionInfoRequest: {:?}", req);

    let missions = match get_event_mission_info(pool, uid).await {
        Ok(list) => list,
        Err(err) => {
            eprintln!("get_event_mission_info failed: {:?}", err);
            vec![]
        }
    };

    let mission_info = missions
        .into_iter()
        .map(|m| EventMissionDbInfo {
            event_id: m.event_id,
            group_id: m.group_id,
            id: m.id,
            value: m.value,
            is_complete: m.is_complete,
        })
        .collect();

    let response = EventMissionInfoResponse {
        mission_info,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let achievement_update_info = vec![AchievementUpdateInfo {
        group_id: Some(101),
        value: Some(0),
        is_set: Some(true),
    }];

    let notify = Notify {
        achievement_update_info,
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

    let (route, code) = PacketCodeType::EventMissionInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
