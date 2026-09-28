use bd2::prost::Message;
use bd2::proto::proto_net::{
    AllCharRefreshRequest, AllCharRefreshResponse, DefineCharStatOption, Notify,
    PictorialBuffStatDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(_pool: &SqlitePool, _uid: i64, req: AllCharRefreshRequest) -> GameResponse {
    info!("Handling AllCharRefreshRequest: {:?}", req);

    let buff_stat_info: Vec<PictorialBuffStatDbInfo> = vec![
        PictorialBuffStatDbInfo {
            stat_type: Some(DefineCharStatOption::HealthPercent as i32),
            stat_value: Some(0.0125),
        },
        PictorialBuffStatDbInfo {
            stat_type: Some(DefineCharStatOption::AttackPercent as i32),
            stat_value: Some(0.0096),
        },
    ];

    let response = AllCharRefreshResponse {
        buff_stat_info,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    // Notify same as others
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2],
        active_contents_info: vec![],
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8916),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::AllCharRefresh.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
