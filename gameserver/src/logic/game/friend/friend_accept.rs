use bd2::prost::Message;
use bd2::proto::proto_net::{FriendAcceptRequest, FriendAcceptResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friend::friend_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real accept: flips both mirrored rows (my "received" row and their "sent" row) to
/// status=0 confirmed.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendAcceptRequest) -> GameResponse {
    info!("Handling FriendAcceptRequest: {:?}", req);

    let mut friend_info = None;
    if let Some(target) = req.owner_index {
        let _ = db::update_status(pool, uid, target, 0).await;
        let _ = db::update_status(pool, target, uid, 0).await;
        friend_info = db::get_by_uid_and_owner(pool, uid, target).await.ok().flatten().map(|r| super::to_proto(&r));
    }

    let response = FriendAcceptResponse { friend_info };
    
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
    
    let (route, code) = PacketCodeType::FriendAccept.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}