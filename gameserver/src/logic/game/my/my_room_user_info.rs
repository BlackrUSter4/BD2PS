use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomUserInfoRequest, MyRoomUserInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{build_user_info, ensure_default_room};

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomUserInfoRequest) -> GameResponse {
    info!("Handling MyRoomUserInfoRequest: {:?}", req);

    let target = req.target_owner_index.unwrap_or(uid);
    if target == uid {
        ensure_default_room(pool, uid).await;
    }

    let room_info = build_user_info(pool, target).await;

    let response = MyRoomUserInfoResponse {
        room_info: Some(room_info),
        sync_use_count: Some(false),
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomUserInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
