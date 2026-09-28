use bd2::prost::Message;
use bd2::proto::proto_net::{UpdateUserContentsInfoOptionRequest, UpdateUserContentsInfoOptionResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::user::user_info as user_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real persistence into the account's real IsAllPrivate/PrivacyOptions columns — this is the
/// "set" counterpart UserContentsInfoOption's own doc comment noted was missing.
pub async fn handle(pool: &SqlitePool, uid: i64, req: UpdateUserContentsInfoOptionRequest) -> GameResponse {
    info!("Handling UpdateUserContentsInfoOptionRequest: {:?}", req);

    let is_all_private = req.is_all_private.unwrap_or(false);
    let _ = user_db::update_contents_info_option(pool, uid, is_all_private, &req.options).await;

    let response = UpdateUserContentsInfoOptionResponse {};
    
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
    
    let (route, code) = PacketCodeType::UpdateUserContentsInfoOption.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}