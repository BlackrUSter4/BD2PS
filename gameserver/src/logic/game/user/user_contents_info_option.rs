use bd2::prost::Message;
use bd2::proto::proto_net::{UserContentsInfoOptionRequest, UserContentsInfoOptionResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::user::user_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account privacy settings (no separate "set" request exists in this stub
/// cluster to populate them, but reads real stored values whenever something else does).
pub async fn handle(pool: &SqlitePool, uid: i64, req: UserContentsInfoOptionRequest) -> GameResponse {
    info!("Handling UserContentsInfoOptionRequest: {:?}", req);

    let row = db::get_user_info(pool, uid).await.ok().and_then(|v| v.into_iter().next());
    let is_all_private = row.as_ref().map(|r| r.is_all_private != 0).unwrap_or(false);
    let options = row
        .and_then(|r| r.privacy_options)
        .and_then(|s| serde_json::from_str::<Vec<i32>>(&s).ok())
        .unwrap_or_default();

    let response = UserContentsInfoOptionResponse { is_all_private: Some(is_all_private), options };
    
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
    
    let (route, code) = PacketCodeType::UserContentsInfoOption.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}