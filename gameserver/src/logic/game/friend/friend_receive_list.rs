use bd2::prost::Message;
use bd2::proto::proto_net::{FriendReceiveListRequest, FriendReceiveListResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friend::friend_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendReceiveListRequest) -> GameResponse {
    info!("Handling FriendReceiveListRequest: {:?}", req);

    let rows = db::get_by_status(pool, uid, 2).await.unwrap_or_default();
    let friend_info = rows.iter().map(super::to_proto).collect();

    let response = FriendReceiveListResponse { friend_info };
    
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
    
    let (route, code) = PacketCodeType::FriendReceiveList.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}