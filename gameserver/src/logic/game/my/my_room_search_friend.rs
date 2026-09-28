use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomSearchFriendRequest, MyRoomSearchFriendResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::build_user_info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomSearchFriendRequest) -> GameResponse {
    info!("Handling MyRoomSearchFriendRequest: {:?}", req);

    let friends = database::db::friend::friend_info::get_friend_info(pool, uid)
        .await
        .unwrap_or_default();

    let mut room_info = Vec::new();
    for friend in friends {
        if let Some(owner) = friend.owner_index {
            room_info.push(build_user_info(pool, owner).await);
        }
    }

    let response = MyRoomSearchFriendResponse { room_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomSearchFriend.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
