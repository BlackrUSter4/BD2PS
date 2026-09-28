use bd2::prost::Message;
use bd2::proto::proto_net::{FriendDbInfo, FriendRecommendRequest, FriendRecommendResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friend::friend_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real recommendations: other real accounts on this server not already in any
/// relationship (confirmed or pending) with this account.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendRecommendRequest) -> GameResponse {
    info!("Handling FriendRecommendRequest: {:?}", req);

    let uids = db::list_recommendable(pool, uid, 20).await.unwrap_or_default();
    let mut friend_info = Vec::with_capacity(uids.len());
    for u in uids {
        friend_info.push(FriendDbInfo {
            owner_index: Some(u),
            user_id: Some(crate::logic::game::display_name(pool, u).await),
            ..Default::default()
        });
    }

    let response = FriendRecommendResponse { friend_info };
    
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
    
    let (route, code) = PacketCodeType::FriendRecommend.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}