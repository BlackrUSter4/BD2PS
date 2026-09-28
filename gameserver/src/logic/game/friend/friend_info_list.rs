use bd2::prost::Message;
use bd2::proto::proto_net::{FriendInfoListRequest, FriendInfoListResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friend::friend_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real confirmed/sent/received relationship lists in one call.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendInfoListRequest) -> GameResponse {
    info!("Handling FriendInfoListRequest: {:?}", req);

    let friend_info = db::get_by_status(pool, uid, 0).await.unwrap_or_default().iter().map(super::to_proto).collect();
    let friend_recv = db::get_by_status(pool, uid, 2).await.unwrap_or_default().iter().map(super::to_proto).collect();
    let friend_send = db::get_by_status(pool, uid, 1).await.unwrap_or_default().iter().map(super::to_proto).collect();

    let response = FriendInfoListResponse { friend_info, friend_recv, friend_send };

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

    let (route, code) = PacketCodeType::FriendInfoList.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
