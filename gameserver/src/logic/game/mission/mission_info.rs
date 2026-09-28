use bd2::prost::Message;
use bd2::proto::proto_net::{
    MissionDbInfo, MissionInfoRequest, MissionInfoResponse, MissionSectionRewardDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mission::mission_info::get_mission_info;
use database::db::mission::mission_section_reward_info::get_mission_section_reward_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MissionInfoRequest) -> GameResponse {
    info!("Handling Mission Info Request for user {}: {:?}", uid, req);

    let missions = get_mission_info(pool, uid).await.unwrap_or_default();
    let rewards = get_mission_section_reward_info(pool, uid)
        .await
        .unwrap_or_default();

    let mission_info = missions
        .into_iter()
        .map(|m| MissionDbInfo {
            group_id: m.group_id,
            id: m.id,
            group_type: m.group_type,
            value: m.value,
            is_complete: m.is_complete,
        })
        .collect::<Vec<_>>();

    let mission_section_reward_info = rewards
        .into_iter()
        .map(|r| MissionSectionRewardDbInfo {
            group_type: r.group_type,
            id: r.id,
        })
        .collect::<Vec<_>>();

    let response = MissionInfoResponse {
        mission_info,
        mission_section_reward_info,
        daily_mission_reset_time: Some(1761436800000),
        weekly_mission_reset_time: Some(1761523200000),
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

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

    let (route, code) = PacketCodeType::MissionInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
