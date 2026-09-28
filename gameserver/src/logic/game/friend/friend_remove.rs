use bd2::prost::Message;
use bd2::proto::proto_net::{FriendRemoveRequest, FriendRemoveResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friend::friend_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real unfriend: deletes both mirrored confirmed rows.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendRemoveRequest) -> GameResponse {
    info!("Handling FriendRemoveRequest: {:?}", req);

    if let Some(target) = req.owner_index {
        let _ = db::delete_by_uid_and_owner(pool, uid, target).await;
        let _ = db::delete_by_uid_and_owner(pool, target, uid).await;
    }

    let response = FriendRemoveResponse {};
    
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
    
    let (route, code) = PacketCodeType::FriendRemove.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}