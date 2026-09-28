use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameHubDbInfo, MiniGameHubInfoRequest, MiniGameHubInfoResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(_pool: &SqlitePool, _uid: i64, req: MiniGameHubInfoRequest) -> GameResponse {
    info!("Handling MiniGameHubInfoRequest: {:?}", req);

    let mini_game_hub_info = vec![
        MiniGameHubDbInfo {
            slot: Some(0),
            event_uid: Some(705),
            progress_type: Some(0),
        },
        MiniGameHubDbInfo {
            slot: Some(1),
            event_uid: Some(788),
            progress_type: Some(0),
        },
        MiniGameHubDbInfo {
            slot: Some(2),
            event_uid: Some(870),
            progress_type: Some(0),
        },
        MiniGameHubDbInfo {
            slot: Some(3),
            event_uid: Some(1007),
            progress_type: Some(0),
        },
        MiniGameHubDbInfo {
            slot: Some(4),
            event_uid: Some(1103),
            progress_type: Some(2),
        },
    ];

    let response = MiniGameHubInfoResponse { mini_game_hub_info };

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

    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
