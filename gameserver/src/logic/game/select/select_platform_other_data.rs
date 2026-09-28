use bd2::prost::Message;
use bd2::proto::proto_net::{SelectPlatformOtherDataRequest, SelectPlatformOtherDataResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::user::user_info as user_db;
use sqlx::SqlitePool;
use tracing::info;

/// Account-linking chooser (merge a platform account vs. keep the guest one). This server has
/// no separate platform-linked-account concept — both slots real-echo the same real account,
/// which is the only sane choice for a single-account private server.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SelectPlatformOtherDataRequest) -> GameResponse {
    info!("Handling SelectPlatformOtherDataRequest: {:?}", req);

    let user_info = user_db::get_user_info(pool, uid).await.unwrap_or_default().into_iter().next().map(|r| r.to_proto());

    let response = SelectPlatformOtherDataResponse {
        platform_user_info: user_info.clone(),
        guest_user_info: user_info,
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
    
    let (route, code) = PacketCodeType::SelectPlatformOtherData.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}