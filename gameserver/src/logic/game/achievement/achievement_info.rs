use bd2::prost::Message;
use bd2::proto::proto_net::{
    AchievementDbInfo, AchievementInfoRequest, AchievementInfoResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::achievement::achievement_info::get_achievement_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: AchievementInfoRequest) -> GameResponse {
    info!("Handling AchievementInfoRequest: {:?}", req);

    let achievements = match get_achievement_info(pool, uid).await {
        Ok(list) => list,
        Err(err) => {
            eprintln!("get_achievement_info failed: {:?}", err);
            vec![]
        }
    };

    let achievement_info = achievements
        .into_iter()
        .map(|a| AchievementDbInfo {
            group_id: a.group_id,
            value: a.value,
            max_clear_id: a.max_clear_id,
            contents_group: a.contents_group,
        })
        .collect();

    let response = AchievementInfoResponse {
        achievement_info,
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

    let (route, code) = PacketCodeType::AchievementInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
